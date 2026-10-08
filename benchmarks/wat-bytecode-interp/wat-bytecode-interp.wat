;; wat-bytecode-interp: a stack-machine bytecode interpreter, hand-written as a
;; WebAssembly component. It reads a bytecode program and its inputs from
;; `default.input`, interprets it between `bench.start` and `bench.end`, and
;; prints the result. See README.md for the bytecode format.

(component
  ;; Host imports: the bench timing hooks and the slice of WASI used.
  (import "bench"
    (instance $bench
      (export "start" (func))
      (export "end" (func))
    )
  )

  (import "wasi:io/error@0.2.6"
    (instance $io-error
      (export "error" (type $error-res (sub resource)))
    )
  )
  (alias export $io-error "error" (type $error))

  (import "wasi:io/streams@0.2.6"
    (instance $streams
      (alias outer 1 $error (type $error-ref))
      (export "error" (type $streams-error (eq $error-ref)))
      (export "output-stream" (type $output-stream (sub resource)))
      (type $stream-error'
        (variant (case "last-operation-failed" (own $streams-error)) (case "closed"))
      )
      (export "stream-error" (type $stream-error (eq $stream-error')))
      (export "[method]output-stream.blocking-write-and-flush"
        (func (param "self" (borrow $output-stream)) (param "contents" (list u8))
          (result (result (error $stream-error)))
        )
      )
    )
  )
  (alias export $streams "output-stream" (type $output-stream))

  (import "wasi:cli/stdout@0.2.6"
    (instance $stdout
      (alias outer 1 $output-stream (type $os-ref))
      (export "output-stream" (type (eq $os-ref)))
      (export "get-stdout" (func (result (own $os-ref))))
    )
  )

  (import "wasi:filesystem/types@0.2.6"
    (instance $fs-types
      (export "descriptor" (type $descriptor (sub resource)))
      (type $error-code'
        (enum "access" "would-block" "already" "bad-descriptor" "busy" "deadlock"
              "quota" "exist" "file-too-large" "illegal-byte-sequence" "in-progress"
              "interrupted" "invalid" "io" "is-directory" "loop" "too-many-links"
              "message-size" "name-too-long" "no-device" "no-entry" "no-lock"
              "insufficient-memory" "insufficient-space" "not-directory" "not-empty"
              "not-recoverable" "unsupported" "no-tty" "no-such-device" "overflow"
              "not-permitted" "pipe" "read-only" "invalid-seek" "text-file-busy"
              "cross-device")
      )
      (export "error-code" (type $error-code (eq $error-code')))
      (type $path-flags' (flags "symlink-follow"))
      (export "path-flags" (type $path-flags (eq $path-flags')))
      (type $open-flags' (flags "create" "directory" "exclusive" "truncate"))
      (export "open-flags" (type $open-flags (eq $open-flags')))
      (type $descriptor-flags'
        (flags "read" "write" "file-integrity-sync" "data-integrity-sync"
               "requested-write-sync" "mutate-directory")
      )
      (export "descriptor-flags" (type $descriptor-flags (eq $descriptor-flags')))
      (export "[method]descriptor.open-at"
        (func (param "self" (borrow $descriptor)) (param "path-flags" $path-flags)
          (param "path" string) (param "open-flags" $open-flags)
          (param "flags" $descriptor-flags)
          (result (result (own $descriptor) (error $error-code)))
        )
      )
      (export "[method]descriptor.read"
        (func (param "self" (borrow $descriptor)) (param "length" u64)
          (param "offset" u64)
          (result (result (tuple (list u8) bool) (error $error-code)))
        )
      )
    )
  )
  (alias export $fs-types "descriptor" (type $descriptor))

  (import "wasi:filesystem/preopens@0.2.6"
    (instance $preopens
      (alias outer 1 $descriptor (type $desc-ref))
      (export "descriptor" (type (eq $desc-ref)))
      (export "get-directories"
        (func (result (list (tuple (own $desc-ref) string))))
      )
    )
  )


  ;; A standalone module owns the memory so that the lowered WASI imports can
  ;; reference it without a cyclic dependency on `$main`, which imports them.
  ;; Pages 0-2 belong to `$main`; host allocations start at page 3.
  (core module $mem
    (memory (export "memory") 4)
    (global $bump (mut i32) (i32.const 0x30000))
    (func (export "realloc")
      (param $old_ptr i32) (param $old_size i32) (param $align i32) (param $new_size i32)
      (result i32)
      (local $ret i32)
      (global.set $bump
        (i32.and
          (i32.add (global.get $bump) (i32.sub (local.get $align) (i32.const 1)))
          (i32.xor (i32.sub (local.get $align) (i32.const 1)) (i32.const -1))
        )
      )
      (local.set $ret (global.get $bump))
      (global.set $bump (i32.add (global.get $bump) (local.get $new_size)))
      (block $done
        (loop $grow
          (br_if $done
            (i32.le_u (global.get $bump)
                      (i32.mul (memory.size) (i32.const 65536)))
          )
          (if (i32.eq (memory.grow (i32.const 1)) (i32.const -1))
            (then (unreachable))
          )
          (br $grow)
        )
      )
      (local.get $ret)
    )
  )
  (core instance $mem (instantiate $mem))

  (core func $c_bench_start (canon lower (func $bench "start")))
  (core func $c_bench_end   (canon lower (func $bench "end")))
  (core func $c_getdirs
    (canon lower (func $preopens "get-directories")
      (memory (core memory $mem "memory")) (realloc (core func $mem "realloc"))
    )
  )
  (core func $c_open
    (canon lower (func $fs-types "[method]descriptor.open-at")
      (memory (core memory $mem "memory"))
    )
  )
  (core func $c_read
    (canon lower (func $fs-types "[method]descriptor.read")
      (memory (core memory $mem "memory")) (realloc (core func $mem "realloc"))
    )
  )
  (core func $c_get_stdout (canon lower (func $stdout "get-stdout")))
  (core func $c_write
    (canon lower (func $streams "[method]output-stream.blocking-write-and-flush")
      (memory (core memory $mem "memory"))
    )
  )

  ;; Memory layout:
  ;;   0x0000..0x0040 : WASI result-pointer scratch
  ;;   0x0040..0x0080 : "default.input" path
  ;;   0x0080..0x0100 : number-formatting scratch
  ;;   0x0100..0x0200 : stdout buffer
  ;;   0x0200..0x0400 : constant strings and the fallback input
  ;;   0x1000..0x10000: bytecode, one i32 per word
  ;;   0x10000..0x20000: operand stack
  ;;   0x20000..0x30000: control stack of (return pc, saved fp) pairs
  (core module $main
    (import "mem" "memory" (memory 4))
    (import "lower" "bench-start" (func $bench_start))
    (import "lower" "bench-end"   (func $bench_end))
    (import "lower" "getdirs"    (func $getdirs (param i32)))
    (import "lower" "open-at"    (func $open_at (param i32 i32 i32 i32 i32 i32 i32)))
    (import "lower" "read"       (func $read (param i32 i64 i64 i32)))
    (import "lower" "get-stdout" (func $get_stdout (result i32)))
    (import "lower" "write"      (func $write (param i32 i32 i32 i32)))

    (global $code_base i32 (i32.const 0x1000))
    (global $code_end i32 (i32.const 0x10000))
    (global $stack_base i32 (i32.const 0x10000))
    (global $stack_end i32 (i32.const 0x20000))
    (global $cstack_base i32 (i32.const 0x20000))
    (global $cstack_end i32 (i32.const 0x30000))

    ;; Input parse cursor and end.
    (global $cur (mut i32) (i32.const 0))
    (global $end (mut i32) (i32.const 0))

    ;; Stdout write cursor.
    (global $out (mut i32) (i32.const 0x100))

    (data (i32.const 0x40) "default.input")
    (data (i32.const 0x200) "fib(")
    (data (i32.const 0x208) ") = ")
    (data (i32.const 0x20c) "\n")
    ;; Used when `default.input` cannot be opened. Keep in sync with it.
    (data (i32.const 0x220)
      "25 1 "
      "9 4 1 0 "
      "2 0 1 2 6 7 14 2 0 10 "
      "2 0 1 1 5 9 4 1 2 0 1 2 5 9 4 1 4 10"
    )
    (global $fallback_ptr i32 (i32.const 0x220))
    (global $fallback_len i32 (i32.const 71))

    ;; Advance past whitespace and `#` line comments. Returns whether a token
    ;; follows.
    (func $skip_ws (result i32)
      (local $ch i32)
      (block $done
        (loop $loop
          (br_if $done (i32.ge_u (global.get $cur) (global.get $end)))
          (local.set $ch (i32.load8_u (global.get $cur)))
          (if (i32.eq (local.get $ch) (i32.const 35))   ;; '#'
            (then
              (block $eol
                (loop $comment
                  (br_if $eol (i32.ge_u (global.get $cur) (global.get $end)))
                  (br_if $eol (i32.eq (i32.load8_u (global.get $cur)) (i32.const 10)))
                  (global.set $cur (i32.add (global.get $cur) (i32.const 1)))
                  (br $comment)
                )
              )
              (br $loop)
            )
          )
          (br_if $done (i32.gt_u (local.get $ch) (i32.const 32)))
          (global.set $cur (i32.add (global.get $cur) (i32.const 1)))
          (br $loop)
        )
      )
      (i32.lt_u (global.get $cur) (global.get $end))
    )

    ;; Parse an optionally negative decimal integer at the cursor, trapping on
    ;; malformed input.
    (func $parse_int (result i32)
      (local $ch i32)
      (local $neg i32)
      (local $val i32)
      (local $digits i32)
      (if (i32.eqz (call $skip_ws)) (then (unreachable)))
      (if (i32.eq (i32.load8_u (global.get $cur)) (i32.const 45))   ;; '-'
        (then
          (local.set $neg (i32.const 1))
          (global.set $cur (i32.add (global.get $cur) (i32.const 1)))
        )
      )
      (block $done
        (loop $loop
          (br_if $done (i32.ge_u (global.get $cur) (global.get $end)))
          (local.set $ch (i32.load8_u (global.get $cur)))
          (br_if $done (i32.le_u (local.get $ch) (i32.const 32)))
          (local.set $ch (i32.sub (local.get $ch) (i32.const 48)))
          (if (i32.gt_u (local.get $ch) (i32.const 9)) (then (unreachable)))
          (local.set $val
            (i32.add (i32.mul (local.get $val) (i32.const 10)) (local.get $ch))
          )
          (local.set $digits (i32.add (local.get $digits) (i32.const 1)))
          (global.set $cur (i32.add (global.get $cur) (i32.const 1)))
          (br $loop)
        )
      )
      (if (i32.eqz (local.get $digits)) (then (unreachable)))
      (if (result i32) (local.get $neg)
        (then (i32.sub (i32.const 0) (local.get $val)))
        (else (local.get $val))
      )
    )

    ;; Bump the operand stack pointer for a push, trapping on overflow.
    (func $push_slot (param $sp i32) (result i32)
      (local.set $sp (i32.add (local.get $sp) (i32.const 4)))
      (if (i32.ge_u (local.get $sp) (global.get $stack_end)) (then (unreachable)))
      (local.get $sp)
    )

    ;; Run the bytecode from word 0 with `$arg` as the only value on the stack
    ;; and local 0 of the outermost frame. Returns the top of stack at `halt`.
    (func $interp (param $arg i32) (result i32)
      (local $pc i32)    ;; address of the current instruction
      (local $sp i32)    ;; address of the top of the operand stack
      (local $fp i32)    ;; address of the current frame's local 0
      (local $csp i32)   ;; next free control stack slot
      (local $v i32)

      (local.set $pc (global.get $code_base))
      (local.set $sp (global.get $stack_base))
      (local.set $fp (global.get $stack_base))
      (local.set $csp (global.get $cstack_base))
      (i32.store (local.get $sp) (local.get $arg))

      (block $halt
        (loop $dispatch
          (block $bad
          (block $jnz
          (block $eq
          (block $mul
          (block $drop
          (block $dup
          (block $ret
          (block $call
          (block $jmp
          (block $jz
          (block $lt
          (block $sub
          (block $add
          (block $store
          (block $load
          (block $push
          (block $halt_op
            (br_table $halt_op $push $load $store $add $sub $lt $jz $jmp $call
                      $ret $dup $drop $mul $eq $jnz $bad
              (i32.load (local.get $pc)))
          )
          ;; 0: halt
          (br $halt)
          )
          ;; 1: push imm
          (local.set $sp (call $push_slot (local.get $sp)))
          (i32.store (local.get $sp) (i32.load offset=4 (local.get $pc)))
          (local.set $pc (i32.add (local.get $pc) (i32.const 8)))
          (br $dispatch)
          )
          ;; 2: load idx
          (local.set $sp (call $push_slot (local.get $sp)))
          (i32.store (local.get $sp)
            (i32.load
              (i32.add (local.get $fp)
                (i32.shl (i32.load offset=4 (local.get $pc)) (i32.const 2)))))
          (local.set $pc (i32.add (local.get $pc) (i32.const 8)))
          (br $dispatch)
          )
          ;; 3: store idx
          (i32.store
            (i32.add (local.get $fp)
              (i32.shl (i32.load offset=4 (local.get $pc)) (i32.const 2)))
            (i32.load (local.get $sp)))
          (local.set $sp (i32.sub (local.get $sp) (i32.const 4)))
          (local.set $pc (i32.add (local.get $pc) (i32.const 8)))
          (br $dispatch)
          )
          ;; 4: add
          (local.set $sp (i32.sub (local.get $sp) (i32.const 4)))
          (i32.store (local.get $sp)
            (i32.add (i32.load (local.get $sp)) (i32.load offset=4 (local.get $sp))))
          (local.set $pc (i32.add (local.get $pc) (i32.const 4)))
          (br $dispatch)
          )
          ;; 5: sub
          (local.set $sp (i32.sub (local.get $sp) (i32.const 4)))
          (i32.store (local.get $sp)
            (i32.sub (i32.load (local.get $sp)) (i32.load offset=4 (local.get $sp))))
          (local.set $pc (i32.add (local.get $pc) (i32.const 4)))
          (br $dispatch)
          )
          ;; 6: lt (signed)
          (local.set $sp (i32.sub (local.get $sp) (i32.const 4)))
          (i32.store (local.get $sp)
            (i32.lt_s (i32.load (local.get $sp)) (i32.load offset=4 (local.get $sp))))
          (local.set $pc (i32.add (local.get $pc) (i32.const 4)))
          (br $dispatch)
          )
          ;; 7: jz target
          (local.set $v (i32.load (local.get $sp)))
          (local.set $sp (i32.sub (local.get $sp) (i32.const 4)))
          (local.set $pc
            (if (result i32) (local.get $v)
              (then (i32.add (local.get $pc) (i32.const 8)))
              (else
                (i32.add (global.get $code_base)
                  (i32.shl (i32.load offset=4 (local.get $pc)) (i32.const 2))))))
          (br $dispatch)
          )
          ;; 8: jmp target
          (local.set $pc
            (i32.add (global.get $code_base)
              (i32.shl (i32.load offset=4 (local.get $pc)) (i32.const 2))))
          (br $dispatch)
          )
          ;; 9: call target nargs
          ;; The new frame's locals start at the first of the `nargs` topmost
          ;; operands. Checking both stacks here bounds recursion.
          (if (i32.or
                (i32.ge_u (local.get $csp) (global.get $cstack_end))
                (i32.ge_u (local.get $sp) (i32.sub (global.get $stack_end) (i32.const 1024))))
            (then (unreachable)))
          (i32.store (local.get $csp) (i32.add (local.get $pc) (i32.const 12)))
          (i32.store offset=4 (local.get $csp) (local.get $fp))
          (local.set $csp (i32.add (local.get $csp) (i32.const 8)))
          (local.set $fp
            (i32.sub (i32.add (local.get $sp) (i32.const 4))
              (i32.shl (i32.load offset=8 (local.get $pc)) (i32.const 2))))
          (local.set $pc
            (i32.add (global.get $code_base)
              (i32.shl (i32.load offset=4 (local.get $pc)) (i32.const 2))))
          (br $dispatch)
          )
          ;; 10: ret
          ;; Replace the frame's locals and operands with the return value.
          (i32.store (local.get $fp) (i32.load (local.get $sp)))
          (local.set $sp (local.get $fp))
          (local.set $csp (i32.sub (local.get $csp) (i32.const 8)))
          (local.set $pc (i32.load (local.get $csp)))
          (local.set $fp (i32.load offset=4 (local.get $csp)))
          (br $dispatch)
          )
          ;; 11: dup
          (local.set $v (i32.load (local.get $sp)))
          (local.set $sp (call $push_slot (local.get $sp)))
          (i32.store (local.get $sp) (local.get $v))
          (local.set $pc (i32.add (local.get $pc) (i32.const 4)))
          (br $dispatch)
          )
          ;; 12: drop
          (local.set $sp (i32.sub (local.get $sp) (i32.const 4)))
          (local.set $pc (i32.add (local.get $pc) (i32.const 4)))
          (br $dispatch)
          )
          ;; 13: mul
          (local.set $sp (i32.sub (local.get $sp) (i32.const 4)))
          (i32.store (local.get $sp)
            (i32.mul (i32.load (local.get $sp)) (i32.load offset=4 (local.get $sp))))
          (local.set $pc (i32.add (local.get $pc) (i32.const 4)))
          (br $dispatch)
          )
          ;; 14: eq
          (local.set $sp (i32.sub (local.get $sp) (i32.const 4)))
          (i32.store (local.get $sp)
            (i32.eq (i32.load (local.get $sp)) (i32.load offset=4 (local.get $sp))))
          (local.set $pc (i32.add (local.get $pc) (i32.const 4)))
          (br $dispatch)
          )
          ;; 15: jnz target
          (local.set $v (i32.load (local.get $sp)))
          (local.set $sp (i32.sub (local.get $sp) (i32.const 4)))
          (local.set $pc
            (if (result i32) (local.get $v)
              (then
                (i32.add (global.get $code_base)
                  (i32.shl (i32.load offset=4 (local.get $pc)) (i32.const 2))))
              (else (i32.add (local.get $pc) (i32.const 8)))))
          (br $dispatch)
          )
          ;; invalid opcode
          (unreachable)
        )
      )
      (i32.load (local.get $sp))
    )

    (func $emit_str (param $ptr i32) (param $len i32)
      (memory.copy (global.get $out) (local.get $ptr) (local.get $len))
      (global.set $out (i32.add (global.get $out) (local.get $len)))
    )

    (func $emit_u32 (param $v i32)
      (local $pos i32)
      (local.set $pos (i32.const 0x100))
      (loop $loop
        (local.set $pos (i32.sub (local.get $pos) (i32.const 1)))
        (i32.store8 (local.get $pos)
          (i32.add (i32.rem_u (local.get $v) (i32.const 10)) (i32.const 48)))
        (local.set $v (i32.div_u (local.get $v) (i32.const 10)))
        (br_if $loop (local.get $v))
      )
      (call $emit_str (local.get $pos) (i32.sub (i32.const 0x100) (local.get $pos)))
    )

    ;; The WASI CLI entry point. Returns 0 on success (lifted to ok(())).
    (func (export "run") (result i32)
      (local $n i32)
      (local $reps i32)
      (local $result i32)
      (local $code i32)
      (local $fd i32)
      (local $off i64)
      (local $len i32)

      ;; Result-pointer scratch slots:
      ;;   0x00: get-directories list
      ;;   0x08: open-at result
      ;;   0x10: read result
      ;;   0x20: blocking-write-and-flush result
      (global.set $cur (global.get $fallback_ptr))
      (global.set $end (i32.add (global.get $fallback_ptr) (global.get $fallback_len)))
      (call $getdirs (i32.const 0))
      (if (i32.load (i32.const 4))
        (then
          (call $open_at
            (i32.load (i32.load (i32.const 0)))   ;; first preopen
            (i32.const 0)                         ;; path-flags
            (i32.const 0x40) (i32.const 13)       ;; path
            (i32.const 0)                         ;; open-flags
            (i32.const 1)                         ;; descriptor-flags: read
            (i32.const 0x08))
          (if (i32.eqz (i32.load (i32.const 0x08)))
            (then
              (local.set $fd (i32.load (i32.const 0x0c)))
              ;; Read until EOF. Each chunk lands right after the previous one,
              ;; since `realloc` bump-allocates and nothing else allocates in
              ;; between.
              (local.set $off (i64.const 0))
              (loop $more
                (call $read (local.get $fd) (i64.const 65536) (local.get $off)
                  (i32.const 0x10))
                (if (i32.load (i32.const 0x10)) (then (unreachable)))
                (local.set $len (i32.load (i32.const 0x18)))
                (if (i64.eqz (local.get $off))
                  (then (global.set $cur (i32.load (i32.const 0x14))))
                  (else
                    (if (i32.ne (i32.load (i32.const 0x14)) (global.get $end))
                      (then (unreachable)))))
                (global.set $end (i32.add (i32.load (i32.const 0x14)) (local.get $len)))
                (local.set $off (i64.add (local.get $off) (i64.extend_i32_u (local.get $len))))
                (br_if $more (i32.and
                  (i32.eqz (i32.load8_u (i32.const 0x1c)))
                  (i32.ne (local.get $len) (i32.const 0)))))))))

      (local.set $n (call $parse_int))
      (local.set $reps (call $parse_int))
      (local.set $code (global.get $code_base))
      (block $done
        (loop $loop
          (br_if $done (i32.eqz (call $skip_ws)))
          (if (i32.ge_u (local.get $code) (global.get $code_end)) (then (unreachable)))
          (i32.store (local.get $code) (call $parse_int))
          (local.set $code (i32.add (local.get $code) (i32.const 4)))
          (br $loop)
        )
      )

      (call $bench_start)
      (block $done
        (loop $loop
          (br_if $done (i32.eqz (local.get $reps)))
          (local.set $result (call $interp (local.get $n)))
          (local.set $reps (i32.sub (local.get $reps) (i32.const 1)))
          (br $loop)
        )
      )
      (call $bench_end)

      (call $emit_str (i32.const 0x200) (i32.const 4))
      (call $emit_u32 (local.get $n))
      (call $emit_str (i32.const 0x208) (i32.const 4))
      (call $emit_u32 (local.get $result))
      (call $emit_str (i32.const 0x20c) (i32.const 1))
      (call $write
        (call $get_stdout)
        (i32.const 0x100)
        (i32.sub (global.get $out) (i32.const 0x100))
        (i32.const 0x20))
      (i32.const 0)
    )
  )

  (core instance $main
    (instantiate $main
      (with "mem" (instance $mem))
      (with "lower"
        (instance
          (export "bench-start" (func $c_bench_start))
          (export "bench-end"   (func $c_bench_end))
          (export "getdirs"     (func $c_getdirs))
          (export "open-at"     (func $c_open))
          (export "read"        (func $c_read))
          (export "get-stdout"  (func $c_get_stdout))
          (export "write"       (func $c_write))
        )
      )
    )
  )

  (func $run (result (result))
    (canon lift (core func $main "run"))
  )
  (instance $run-iface (export "run" (func $run)))
  (export "wasi:cli/run@0.2.6" (instance $run-iface))
)
