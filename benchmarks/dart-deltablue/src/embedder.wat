;; Wasm implementations of the `dart.*` host imports that dart2wasm's
;; standalone mode emits for this program, plus the `_start` entry point, so
;; that the linked module needs only WASI and `bench`. `compile.sh` links it into
;; the Dart module with `wasm-merge`; a missing import fails the build.
;;
;; Embedder strings are `(array (mut i8))` byte strings passed as externref.
;; The program only creates strings from ASCII bytes and integers, so code
;; units fit in a byte.
(module
  (type $bytes (array (mut i8)))
  (type $args (array (mut externref)))
  (type $sb (struct (field $data (mut (ref $bytes))) (field $len (mut i32))))

  (import "wasi_snapshot_preview1" "fd_write"
    (func $fd_write (param i32 i32 i32 i32) (result i32)))
  (import "ffi" "memory" (memory 1))
  (import "main" "$invokeMain" (func $invokeMain (param (ref $args))))

  ;; The Dart driver uses memory below 2048 for its own WASI calls.
  (global $iov i32 (i32.const 2048))
  (global $nwritten i32 (i32.const 2056))
  (global $out i32 (i32.const 4096))
  (global $outCap i32 (i32.const 32768))

  (func (export "_start")
    (call $invokeMain (array.new_fixed $args 0)))

  (func $str (param externref) (result (ref $bytes))
    (ref.cast (ref $bytes) (any.convert_extern (local.get 0))))

  (func $ext (param (ref $bytes)) (result (ref extern))
    (extern.convert_any (local.get 0)))

  (func (export "stringFromAsciiBytes")
    (param $src (ref $bytes)) (param $start i32) (param $len i32)
    (result (ref extern))
    (local $dst (ref $bytes))
    (local.set $dst (array.new_default $bytes (local.get $len)))
    (array.copy $bytes $bytes
      (local.get $dst) (i32.const 0)
      (local.get $src) (local.get $start) (local.get $len))
    (call $ext (local.get $dst)))

  (func (export "stringLength") (param externref) (result i32)
    (array.len (call $str (local.get 0))))

  (func (export "stringCodeUnitAt") (param externref i32) (result i32)
    (array.get_u $bytes (call $str (local.get 0)) (local.get 1)))

  ;; `int.toRadixString`; radix is in [2, 36].
  (func (export "i64ToString") (param $v i64) (param $radix i32)
    (result (ref extern))
    (local $buf (ref $bytes))
    (local $pos i32)
    (local $d i32)
    (local $r i64)
    (local $res (ref $bytes))
    (local.set $buf (array.new_default $bytes (i32.const 65)))
    (local.set $pos (i32.const 65))
    (local.set $r (i64.extend_i32_u (local.get $radix)))
    ;; Digits come from signed remainders so that i64 min needs no negation.
    (loop $digits
      (local.set $d (i32.wrap_i64 (i64.rem_s (local.get $v) (local.get $r))))
      (if (i32.lt_s (local.get $d) (i32.const 0))
        (then (local.set $d (i32.sub (i32.const 0) (local.get $d)))))
      (local.set $pos (i32.sub (local.get $pos) (i32.const 1)))
      (array.set $bytes (local.get $buf) (local.get $pos)
        (select
          (i32.add (local.get $d) (i32.const 0x30))
          (i32.add (local.get $d) (i32.const 0x57))
          (i32.lt_u (local.get $d) (i32.const 10))))
      (local.set $v (i64.div_s (local.get $v) (local.get $r)))
      (br_if $digits (i64.ne (local.get $v) (i64.const 0))))
    (if (i32.wrap_i64 (i64.shr_u (local.get 0) (i64.const 63)))
      (then
        (local.set $pos (i32.sub (local.get $pos) (i32.const 1)))
        (array.set $bytes (local.get $buf) (local.get $pos) (i32.const 0x2d))))
    (local.set $res
      (array.new_default $bytes (i32.sub (i32.const 65) (local.get $pos))))
    (array.copy $bytes $bytes
      (local.get $res) (i32.const 0)
      (local.get $buf) (local.get $pos) (i32.sub (i32.const 65) (local.get $pos)))
    (call $ext (local.get $res)))

  (func (export "stringBufferCreate") (result (ref extern))
    (extern.convert_any
      (struct.new $sb (array.new_default $bytes (i32.const 16)) (i32.const 0))))

  (func (export "stringBufferWriteString") (param externref externref)
    (local $sb (ref $sb))
    (local $s (ref $bytes))
    (local $len i32)
    (local $need i32)
    (local $cap i32)
    (local $grown (ref $bytes))
    (local.set $sb (ref.cast (ref $sb) (any.convert_extern (local.get 0))))
    (local.set $s (call $str (local.get 1)))
    (local.set $len (struct.get $sb $len (local.get $sb)))
    (local.set $need (i32.add (local.get $len) (array.len (local.get $s))))
    (local.set $cap (array.len (struct.get $sb $data (local.get $sb))))
    (if (i32.gt_u (local.get $need) (local.get $cap))
      (then
        (loop $grow
          (local.set $cap (i32.shl (local.get $cap) (i32.const 1)))
          (br_if $grow (i32.gt_u (local.get $need) (local.get $cap))))
        (local.set $grown (array.new_default $bytes (local.get $cap)))
        (array.copy $bytes $bytes
          (local.get $grown) (i32.const 0)
          (struct.get $sb $data (local.get $sb)) (i32.const 0) (local.get $len))
        (struct.set $sb $data (local.get $sb) (local.get $grown))))
    (array.copy $bytes $bytes
      (struct.get $sb $data (local.get $sb)) (local.get $len)
      (local.get $s) (i32.const 0) (array.len (local.get $s)))
    (struct.set $sb $len (local.get $sb) (local.get $need)))

  (func (export "stringBufferToString") (param externref) (result (ref extern))
    (local $sb (ref $sb))
    (local $res (ref $bytes))
    (local.set $sb (ref.cast (ref $sb) (any.convert_extern (local.get 0))))
    (local.set $res (array.new_default $bytes (struct.get $sb $len (local.get $sb))))
    (array.copy $bytes $bytes
      (local.get $res) (i32.const 0)
      (struct.get $sb $data (local.get $sb)) (i32.const 0)
      (struct.get $sb $len (local.get $sb)))
    (call $ext (local.get $res)))

  ;; Stack traces are only rendered for uncaught exceptions; leave them empty.
  (func (export "stackTraceGetCurrent") (result (ref extern))
    (call $ext (array.new_fixed $bytes 0)))

  (func (export "stackTraceToString") (param externref) (result (ref extern))
    (call $ext (call $str (local.get 0))))

  ;; Writes `line` and a newline to stdout, in `$outCap`-sized chunks.
  (func (export "print") (param $line externref)
    (local $s (ref $bytes))
    (local $len i32)
    (local $off i32)
    (local $n i32)
    (local $i i32)
    (local $last i32)
    (local.set $s (call $str (local.get $line)))
    (local.set $len (array.len (local.get $s)))
    (loop $chunk
      (local.set $n (i32.sub (local.get $len) (local.get $off)))
      (local.set $last (i32.lt_u (local.get $n) (global.get $outCap)))
      (if (i32.eqz (local.get $last))
        (then (local.set $n (global.get $outCap))))
      (local.set $i (i32.const 0))
      (block $copied
        (loop $copy
          (br_if $copied (i32.ge_u (local.get $i) (local.get $n)))
          (i32.store8
            (i32.add (global.get $out) (local.get $i))
            (array.get_u $bytes (local.get $s)
              (i32.add (local.get $off) (local.get $i))))
          (local.set $i (i32.add (local.get $i) (i32.const 1)))
          (br $copy)))
      (local.set $off (i32.add (local.get $off) (local.get $n)))
      (if (local.get $last)
        (then
          (i32.store8 (i32.add (global.get $out) (local.get $n)) (i32.const 10))
          (local.set $n (i32.add (local.get $n) (i32.const 1)))))
      (call $writeAll (global.get $out) (local.get $n))
      (br_if $chunk (i32.eqz (local.get $last)))))

  (func $writeAll (param $ptr i32) (param $len i32)
    (block $done
      (loop $write
        (br_if $done (i32.eqz (local.get $len)))
        (i32.store (global.get $iov) (local.get $ptr))
        (i32.store offset=4 (global.get $iov) (local.get $len))
        (br_if $done
          (call $fd_write
            (i32.const 1) (global.get $iov) (i32.const 1) (global.get $nwritten)))
        (local.set $ptr (i32.add (local.get $ptr) (i32.load (global.get $nwritten))))
        (local.set $len (i32.sub (local.get $len) (i32.load (global.get $nwritten))))
        (br $write))))
)
