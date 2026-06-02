;; cm-online-stats: a WebAssembly *component* benchmark for sightglass.
;;
;; The root component wires together three sub-components:
;;
;;   1. `$stats`  - defines and exports an `online-stats` resource that computes
;;                  the running mean, standard deviation, min, and max of a
;;                  stream of f64 samples using Welford's online algorithm. The
;;                  resource representation is a pointer into the sub-component's
;;                  own linear memory, bump-allocated in `new-online-stats`.
;;
;;   2. `$rng`    - a global xorshift128+ pseudo-random number generator. It
;;                  exports `seed` and `next-f64`; its state lives in module
;;                  globals rather than a resource.
;;
;;   3. `$runner` - imports the `$stats` and `$rng` instances plus the `bench`
;;                  timing hooks and the slice of WASI it needs, and exports a
;;                  `run` function compatible with the WASI CLI `command` world.
;;                  It reads `SEED,ITERS` from `default.input`, seeds the PRNG,
;;                  feeds `ITERS` random samples in [0, 1) to the stats resource,
;;                  and prints the resulting statistics to stdout.
;;
;; The root re-exports the runner's `run` as `wasi:cli/run@0.2.6`.

(component

  ;; =====================================================================
  ;; Sub-component 1: the `online-stats` resource provider.
  ;; =====================================================================
  (component $stats

    ;; Core module implementing the statistics over linear memory. Each
    ;; `online-stats` object is 40 bytes laid out as:
    ;;   +0 : count (i64)
    ;;   +8 : mean  (f64)
    ;;   +16: M2    (f64)  - running sum of squares of deltas (Welford)
    ;;   +24: min   (f64)
    ;;   +32: max   (f64)
    (core module $stats_core
      ;; `resource.new` intrinsic: turns a rep (i32 pointer) into a handle.
      (import "intr" "new" (func $new_handle (param i32) (result i32)))

      (memory 1)

      ;; Bump allocator for stats objects. Starts at 8 to keep 0 reserved.
      (global $bump (mut i32) (i32.const 8))

      ;; new-online-stats: allocate and zero/seed a fresh stats object,
      ;; returning an owned handle.
      (func (export "new") (result i32)
        (local $p i32)
        (local.set $p (global.get $bump))
        (global.set $bump (i32.add (local.get $p) (i32.const 40)))
        (i64.store           (local.get $p) (i64.const 0))       ;; count
        (f64.store offset=8  (local.get $p) (f64.const 0))       ;; mean
        (f64.store offset=16 (local.get $p) (f64.const 0))       ;; M2
        (f64.store offset=24 (local.get $p) (f64.const inf))     ;; min
        (f64.store offset=32 (local.get $p) (f64.const -inf))    ;; max
        (call $new_handle (local.get $p))
      )

      ;; add-sample: incorporate one sample via Welford's algorithm.
      (func (export "add") (param $p i32) (param $x f64)
        (local $count f64)
        (local $mean f64)
        (local $delta f64)
        (local $m2 f64)
        ;; count += 1
        (local.set $count
          (f64.convert_i64_u
            (i64.add (i64.load (local.get $p)) (i64.const 1))
          )
        )
        (i64.store (local.get $p) (i64.trunc_f64_u (local.get $count)))
        ;; delta = x - mean
        (local.set $mean (f64.load offset=8 (local.get $p)))
        (local.set $delta (f64.sub (local.get $x) (local.get $mean)))
        ;; mean += delta / count
        (local.set $mean
          (f64.add (local.get $mean) (f64.div (local.get $delta) (local.get $count)))
        )
        (f64.store offset=8 (local.get $p) (local.get $mean))
        ;; M2 += delta * (x - mean)   (mean already updated)
        (local.set $m2 (f64.load offset=16 (local.get $p)))
        (f64.store offset=16 (local.get $p)
          (f64.add (local.get $m2)
            (f64.mul (local.get $delta) (f64.sub (local.get $x) (local.get $mean)))
          )
        )
        ;; min = min(min, x)
        (if (f64.lt (local.get $x) (f64.load offset=24 (local.get $p)))
          (then (f64.store offset=24 (local.get $p) (local.get $x)))
        )
        ;; max = max(max, x)
        (if (f64.gt (local.get $x) (f64.load offset=32 (local.get $p)))
          (then (f64.store offset=32 (local.get $p) (local.get $x)))
        )
      )

      (func (export "mean") (param $p i32) (result f64)
        (f64.load offset=8 (local.get $p))
      )

      ;; Population standard deviation: sqrt(M2 / count). Guards count == 0.
      (func (export "std-dev") (param $p i32) (result f64)
        (local $count i64)
        (local.set $count (i64.load (local.get $p)))
        (if (result f64) (i64.eqz (local.get $count))
          (then
            (f64.const 0)
          )
          (else
            (f64.sqrt
              (f64.div
                (f64.load offset=16 (local.get $p))
                (f64.convert_i64_u (local.get $count))
              )
            )
          )
        )
      )

      (func (export "min") (param $p i32) (result f64)
        (f64.load offset=24 (local.get $p))
      )

      (func (export "max") (param $p i32) (result f64)
        (f64.load offset=32 (local.get $p))
      )
    )

    ;; The exported resource type. Its representation is an i32 (a pointer into
    ;; `$stats_core`'s memory). No destructor is needed: storage is never freed.
    (type $online-stats' (resource (rep i32)))
    (export $online-stats "online-stats" (type $online-stats'))

    ;; `resource.new` intrinsic, fed into the core module.
    (canon resource.new $online-stats' (core func $os_new))
    (core instance $sc
      (instantiate $stats_core
        (with "intr"
          (instance (export "new" (func $os_new)))
        )
      )
    )

    ;; Lift the core functions to the component ABI, threading the resource
    ;; type through owned/borrowed handles.
    (func (export "new-online-stats") (result (own $online-stats))
      (canon lift (core func $sc "new"))
    )
    (func (export "add-sample")
      (param "stats" (borrow $online-stats)) (param "sample" f64)
      (canon lift (core func $sc "add"))
    )
    (func (export "get-mean")
      (param "stats" (borrow $online-stats)) (result f64)
      (canon lift (core func $sc "mean"))
    )
    (func (export "get-std-dev")
      (param "stats" (borrow $online-stats)) (result f64)
      (canon lift (core func $sc "std-dev"))
    )
    (func (export "get-min")
      (param "stats" (borrow $online-stats)) (result f64)
      (canon lift (core func $sc "min"))
    )
    (func (export "get-max")
      (param "stats" (borrow $online-stats)) (result f64)
      (canon lift (core func $sc "max"))
    )
  )

  ;; =====================================================================
  ;; Sub-component 2: the xorshift128+ pseudo-random number generator.
  ;; =====================================================================
  (component $rng

    (core module $rng_core
      ;; xorshift128+ state (128 bits), held in module globals.
      (global $s0 (mut i64) (i64.const 0))
      (global $s1 (mut i64) (i64.const 0))

      ;; splitmix64 finalizer used to seed the generator.
      (func $mix (param $z i64) (result i64)
        (local.set $z
          (i64.mul
            (i64.xor (local.get $z) (i64.shr_u (local.get $z) (i64.const 30)))
            (i64.const 0xBF58476D1CE4E5B9)
          )
        )
        (local.set $z
          (i64.mul
            (i64.xor (local.get $z) (i64.shr_u (local.get $z) (i64.const 27)))
            (i64.const 0x94D049BB133111EB)
          )
        )
        (i64.xor (local.get $z) (i64.shr_u (local.get $z) (i64.const 31)))
      )

      ;; Seed the xorshift128+ state from a single 64-bit seed.
      (func (export "seed") (param $seed i64)
        (local $z i64)
        (local.set $z (i64.add (local.get $seed) (i64.const 0x9E3779B97F4A7C15)))
        (global.set $s0 (call $mix (local.get $z)))
        (local.set $z (i64.add (local.get $z) (i64.const 0x9E3779B97F4A7C15)))
        (global.set $s1 (call $mix (local.get $z)))
      )

      ;; xorshift128+: advance the state and return the next 64-bit value.
      (func $next_u64 (result i64)
        (local $x i64)
        (local $y i64)
        (local $result i64)
        (local.set $x (global.get $s0))
        (local.set $y (global.get $s1))
        (local.set $result (i64.add (local.get $x) (local.get $y)))
        (global.set $s0 (local.get $y))
        (local.set $x (i64.xor (local.get $x) (i64.shl (local.get $x) (i64.const 23))))
        (global.set $s1
          (i64.xor
            (i64.xor (local.get $x) (local.get $y))
            (i64.xor
              (i64.shr_u (local.get $x) (i64.const 18))
              (i64.shr_u (local.get $y) (i64.const 5))
            )
          )
        )
        (local.get $result)
      )

      ;; A uniformly random f64 in [0, 1): take the top 53 bits / 2^53.
      (func (export "next-f64") (result f64)
        (f64.mul
          (f64.convert_i64_u (i64.shr_u (call $next_u64) (i64.const 11)))
          (f64.const 0x1p-53)
        )
      )
    )
    (core instance $rc (instantiate $rng_core))

    (func (export "seed") (param "seed" u64)
      (canon lift (core func $rc "seed"))
    )
    (func (export "next-f64") (result f64)
      (canon lift (core func $rc "next-f64"))
    )
  )

  ;; =====================================================================
  ;; Sub-component 3: the runner / WASI CLI command.
  ;; =====================================================================
  (component $runner

    ;; ---- Import the online-stats instance from sub-component 1 ----
    (import "online-stats"
      (instance $os
        (export "online-stats" (type $os-ty (sub resource)))
        (export "new-online-stats" (func (result (own $os-ty))))
        (export "add-sample"
          (func (param "stats" (borrow $os-ty)) (param "sample" f64))
        )
        (export "get-mean" (func (param "stats" (borrow $os-ty)) (result f64)))
        (export "get-std-dev" (func (param "stats" (borrow $os-ty)) (result f64)))
        (export "get-min" (func (param "stats" (borrow $os-ty)) (result f64)))
        (export "get-max" (func (param "stats" (borrow $os-ty)) (result f64)))
      )
    )

    ;; ---- Import the PRNG instance from sub-component 2 ----
    (import "rng"
      (instance $rng
        (export "seed" (func (param "seed" u64)))
        (export "next-f64" (func (result f64)))
      )
    )

    ;; ---- Import the bench timing hooks ----
    (import "bench"
      (instance $bench
        (export "start" (func))
        (export "end" (func))
      )
    )

    ;; ---- Import the slice of WASI we use ----
    ;; wasi:io/error provides the `error` resource that stream-error references.
    (import "wasi:io/error@0.2.6"
      (instance $io-error
        (export "error" (type $error-res (sub resource)))
      )
    )
    (alias export $io-error "error" (type $error))

    ;; wasi:io/streams: we only need output-stream + blocking-write-and-flush.
    ;; `error` is aliased from wasi:io/error so the linker can match it, and the
    ;; named `stream-error` aggregate is exported (a rule for import instances).
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

    ;; wasi:cli/stdout: get-stdout returns the same output-stream type.
    (import "wasi:cli/stdout@0.2.6"
      (instance $stdout
        (alias outer 1 $output-stream (type $os-ref))
        (export "output-stream" (type (eq $os-ref)))
        (export "get-stdout" (func (result (own $os-ref))))
      )
    )

    ;; wasi:filesystem/types: descriptor + open-at + read.
    (import "wasi:filesystem/types@0.2.6"
      (instance $fs-types
        (export "descriptor" (type $descriptor (sub resource)))
        ;; Named aggregate types used by funcs must be exported from the instance.
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

    ;; wasi:filesystem/preopens: get-directories returns the preopened dirs.
    (import "wasi:filesystem/preopens@0.2.6"
      (instance $preopens
        (alias outer 1 $descriptor (type $desc-ref))
        (export "descriptor" (type (eq $desc-ref)))
        (export "get-directories"
          (func (result (list (tuple (own $desc-ref) string))))
        )
      )
    )

    ;; ---- Linear memory + realloc (provided to the canonical ABI) ----
    ;; A standalone module owns the memory so that the lowered WASI imports can
    ;; reference it without a cyclic dependency on `$main` (which imports them).
    (core module $mem
      (memory (export "memory") 1)
      (global $bump (mut i32) (i32.const 8192))    ;; host allocations live high
      (func (export "realloc")
        (param $old_ptr i32) (param $old_size i32) (param $align i32) (param $new_size i32)
        (result i32)
        (local $ret i32)
        ;; align the bump pointer up to $align
        (global.set $bump
          (i32.and
            (i32.add (global.get $bump) (i32.sub (local.get $align) (i32.const 1)))
            (i32.xor (i32.sub (local.get $align) (i32.const 1)) (i32.const -1))
          )
        )
        (local.set $ret (global.get $bump))
        (global.set $bump (i32.add (global.get $bump) (local.get $new_size)))
        ;; grow memory until $bump fits
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

    ;; ---- Lower component imports into core functions ----
    (core func $c_bench_start (canon lower (func $bench "start")))
    (core func $c_bench_end   (canon lower (func $bench "end")))

    (core func $c_rng_seed (canon lower (func $rng "seed")))
    (core func $c_rng_next (canon lower (func $rng "next-f64")))

    (core func $c_os_new    (canon lower (func $os "new-online-stats")))
    (core func $c_os_add    (canon lower (func $os "add-sample")))
    (core func $c_os_mean   (canon lower (func $os "get-mean")))
    (core func $c_os_stddev (canon lower (func $os "get-std-dev")))
    (core func $c_os_min    (canon lower (func $os "get-min")))
    (core func $c_os_max    (canon lower (func $os "get-max")))

    (core func $c_getdirs
      (canon lower (func $preopens "get-directories")
        (memory $mem "memory") (realloc (func $mem "realloc"))
      )
    )
    (core func $c_open
      (canon lower (func $fs-types "[method]descriptor.open-at")
        (memory $mem "memory")
      )
    )
    (core func $c_read
      (canon lower (func $fs-types "[method]descriptor.read")
        (memory $mem "memory") (realloc (func $mem "realloc"))
      )
    )
    (core func $c_get_stdout
      (canon lower (func $stdout "get-stdout"))
    )
    (core func $c_write
      (canon lower (func $streams "[method]output-stream.blocking-write-and-flush")
        (memory $mem "memory")
      )
    )

    ;; ---- The main runner module ----
    ;; Fixed memory layout (shared with $mem; host allocations start at 8192):
    ;;   0..64  : WASI result-pointer scratch (see `run`)
    ;;   64..77 : "default.input" path
    ;;   96..128: number-formatting scratch
    ;;   128..  : stdout output buffer
    ;;   512..  : label string constants
    (core module $main
      (import "mem" "memory" (memory 1))
      (import "lower" "bench-start" (func $bench_start))
      (import "lower" "bench-end"   (func $bench_end))
      (import "lower" "rng-seed"  (func $rng_seed (param i64)))
      (import "lower" "rng-next"  (func $rng_next (result f64)))
      (import "lower" "os-new"    (func $os_new (result i32)))
      (import "lower" "os-add"    (func $os_add (param i32 f64)))
      (import "lower" "os-mean"   (func $os_mean (param i32) (result f64)))
      (import "lower" "os-stddev" (func $os_stddev (param i32) (result f64)))
      (import "lower" "os-min"    (func $os_min (param i32) (result f64)))
      (import "lower" "os-max"    (func $os_max (param i32) (result f64)))
      (import "lower" "getdirs"   (func $getdirs (param i32)))
      (import "lower" "open-at"   (func $open_at (param i32 i32 i32 i32 i32 i32 i32)))
      (import "lower" "read"      (func $read (param i32 i64 i64 i32)))
      (import "lower" "get-stdout" (func $get_stdout (result i32)))
      (import "lower" "write"     (func $write (param i32 i32 i32 i32)))

      ;; Output buffer write cursor.
      (global $out (mut i32) (i32.const 128))

      ;; Fixed constants.
      (data (i32.const 64) "default.input")
      (data (i32.const 512) "mean ")        ;; +512, len 5
      (data (i32.const 520) "stddev ")      ;; +520, len 7
      (data (i32.const 528) "min ")         ;; +528, len 4
      (data (i32.const 536) "max ")         ;; +536, len 4

      ;; Append `len` bytes from `ptr` to the output buffer.
      (func $emit_str (param $ptr i32) (param $len i32)
        (local $i i32)
        (block $done
          (loop $loop
            (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
            (i32.store8
              (i32.add (global.get $out) (local.get $i))
              (i32.load8_u (i32.add (local.get $ptr) (local.get $i)))
            )
            (local.set $i (i32.add (local.get $i) (i32.const 1)))
            (br $loop)
          )
        )
        (global.set $out (i32.add (global.get $out) (local.get $len)))
      )

      ;; Append a single byte to the output buffer.
      (func $emit_byte (param $b i32)
        (i32.store8 (global.get $out) (local.get $b))
        (global.set $out (i32.add (global.get $out) (i32.const 1)))
      )

      ;; Append the decimal form of an unsigned 64-bit value.
      (func $emit_u64 (param $v i64)
        (local $pos i32)
        (if (i64.eqz (local.get $v))
          (then
            (call $emit_byte (i32.const 48))
            (return)
          )
        )
        ;; Write digits backwards into scratch ending at 128.
        (local.set $pos (i32.const 128))
        (block $done
          (loop $loop
            (br_if $done (i64.eqz (local.get $v)))
            (local.set $pos (i32.sub (local.get $pos) (i32.const 1)))
            (i32.store8 (local.get $pos)
              (i32.wrap_i64
                (i64.add (i64.rem_u (local.get $v) (i64.const 10)) (i64.const 48))
              )
            )
            (local.set $v (i64.div_u (local.get $v) (i64.const 10)))
            (br $loop)
          )
        )
        (call $emit_str (local.get $pos) (i32.sub (i32.const 128) (local.get $pos)))
      )

      ;; Append exactly `width` decimal digits of `v`, zero-padded on the left.
      (func $emit_u64_pad (param $v i64) (param $width i32)
        (local $pos i32)
        (local.set $pos (i32.const 128))
        (block $done
          (loop $loop
            (br_if $done (i32.eqz (local.get $width)))
            (local.set $pos (i32.sub (local.get $pos) (i32.const 1)))
            (i32.store8 (local.get $pos)
              (i32.wrap_i64
                (i64.add (i64.rem_u (local.get $v) (i64.const 10)) (i64.const 48))
              )
            )
            (local.set $v (i64.div_u (local.get $v) (i64.const 10)))
            (local.set $width (i32.sub (local.get $width) (i32.const 1)))
            (br $loop)
          )
        )
        (call $emit_str (local.get $pos) (i32.sub (i32.const 128) (local.get $pos)))
      )

      ;; Append an f64 in [0, 2^64) as "<int>.<9 fractional digits>".
      (func $emit_f64 (param $x f64)
        (local $ip i64)
        (local.set $ip (i64.trunc_f64_u (local.get $x)))
        (call $emit_u64 (local.get $ip))
        (call $emit_byte (i32.const 46))   ;; '.'
        (call $emit_u64_pad
          (i64.trunc_f64_u
            (f64.mul
              (f64.sub (local.get $x) (f64.convert_i64_u (local.get $ip)))
              (f64.const 1e9)
            )
          )
          (i32.const 9)
        )
      )

      ;; Parse one unsigned decimal integer starting at the cursor stored at
      ;; memory[$cur_addr], skipping any leading non-digit bytes. Returns the
      ;; value and updates the stored cursor.
      (func $parse_u64 (param $cur_addr i32) (param $end i32) (result i64)
        (local $cur i32)
        (local $ch i32)
        (local $val i64)
        (local.set $cur (i32.load (local.get $cur_addr)))
        ;; skip leading non-digits
        (block $skipped
          (loop $skip
            (br_if $skipped (i32.ge_u (local.get $cur) (local.get $end)))
            (local.set $ch (i32.load8_u (local.get $cur)))
            (br_if $skipped
              (i32.and
                (i32.ge_u (local.get $ch) (i32.const 48))
                (i32.le_u (local.get $ch) (i32.const 57))
              )
            )
            (local.set $cur (i32.add (local.get $cur) (i32.const 1)))
            (br $skip)
          )
        )
        ;; accumulate digits
        (block $done
          (loop $digits
            (br_if $done (i32.ge_u (local.get $cur) (local.get $end)))
            (local.set $ch (i32.load8_u (local.get $cur)))
            (br_if $done
              (i32.or
                (i32.lt_u (local.get $ch) (i32.const 48))
                (i32.gt_u (local.get $ch) (i32.const 57))
              )
            )
            (local.set $val
              (i64.add
                (i64.mul (local.get $val) (i64.const 10))
                (i64.extend_i32_u (i32.sub (local.get $ch) (i32.const 48)))
              )
            )
            (local.set $cur (i32.add (local.get $cur) (i32.const 1)))
            (br $digits)
          )
        )
        (i32.store (local.get $cur_addr) (local.get $cur))
        (local.get $val)
      )

      ;; The WASI CLI entry point. Returns 0 on success (lifted to ok(())).
      (func (export "run") (result i32)
        (local $preopen i32)
        (local $fd i32)
        (local $bytes i32)
        (local $len i32)
        (local $seed i64)
        (local $iters i64)
        (local $i i64)
        (local $stats i32)
        (local $mean f64)
        (local $stddev f64)
        (local $mn f64)
        (local $mx f64)

        ;; Result-pointer scratch slots (each WASI call uses a distinct one):
        ;;   0  : get-directories list header
        ;;   8  : open-at result
        ;;   16 : read result
        ;;   32 : blocking-write-and-flush result
        ;;   48 : parse cursor

        ;; 1. Find the first preopened directory's descriptor.
        (call $getdirs (i32.const 0))
        (local.set $preopen (i32.load (i32.load (i32.const 0))))

        ;; 2. Open "default.input" for reading relative to that directory.
        (call $open_at
          (local.get $preopen)
          (i32.const 0)        ;; path-flags
          (i32.const 64)       ;; path ptr
          (i32.const 13)       ;; path len
          (i32.const 0)        ;; open-flags
          (i32.const 1)        ;; descriptor-flags: read
          (i32.const 8)        ;; result ptr
        )
        (if (i32.load (i32.const 8)) (then (unreachable)))    ;; err -> trap
        (local.set $fd (i32.load (i32.const 12)))

        ;; 3. Read up to 4096 bytes from offset 0.
        (call $read
          (local.get $fd)
          (i64.const 4096)     ;; length
          (i64.const 0)        ;; offset
          (i32.const 16)       ;; result ptr
        )
        (if (i32.load (i32.const 16)) (then (unreachable)))   ;; err -> trap
        (local.set $bytes (i32.load (i32.const 20)))          ;; list ptr
        (local.set $len (i32.load (i32.const 24)))            ;; list len

        ;; 4. Parse "SEED,ITERS".
        (i32.store (i32.const 48) (local.get $bytes))   ;; cursor
        (local.set $seed
          (call $parse_u64 (i32.const 48) (i32.add (local.get $bytes) (local.get $len)))
        )
        (local.set $iters
          (call $parse_u64 (i32.const 48) (i32.add (local.get $bytes) (local.get $len)))
        )

        ;; 5. Seed the PRNG.
        (call $rng_seed (local.get $seed))

        ;; 6. Start timing.
        (call $bench_start)

        ;; 7. Create the online-stats resource.
        (local.set $stats (call $os_new))

        ;; 8. Feed ITERS random samples.
        (local.set $i (i64.const 0))
        (block $loop_done
          (loop $loop
            (br_if $loop_done (i64.ge_u (local.get $i) (local.get $iters)))
            (call $os_add (local.get $stats) (call $rng_next))
            (local.set $i (i64.add (local.get $i) (i64.const 1)))
            (br $loop)
          )
        )

        ;; 9. Collect the statistics.
        (local.set $mean   (call $os_mean   (local.get $stats)))
        (local.set $stddev (call $os_stddev (local.get $stats)))
        (local.set $mn     (call $os_min    (local.get $stats)))
        (local.set $mx     (call $os_max    (local.get $stats)))

        ;; 10. Stop timing.
        (call $bench_end)

        ;; 11. Format and print the statistics to stdout.
        (global.set $out (i32.const 128))
        (call $emit_str (i32.const 512) (i32.const 5))
        (call $emit_f64 (local.get $mean))
        (call $emit_byte (i32.const 10))
        (call $emit_str (i32.const 520) (i32.const 7))
        (call $emit_f64 (local.get $stddev))
        (call $emit_byte (i32.const 10))
        (call $emit_str (i32.const 528) (i32.const 4))
        (call $emit_f64 (local.get $mn))
        (call $emit_byte (i32.const 10))
        (call $emit_str (i32.const 536) (i32.const 4))
        (call $emit_f64 (local.get $mx))
        (call $emit_byte (i32.const 10))
        (call $write
          (call $get_stdout)
          (i32.const 128)
          (i32.sub (global.get $out) (i32.const 128))
          (i32.const 32)
        )

        ;; 12. Success.
        (i32.const 0)
      )
    )

    ;; Instantiate the runner, wiring the memory and the lowered imports.
    (core instance $main
      (instantiate $main
        (with "mem" (instance $mem))
        (with "lower"
          (instance
            (export "bench-start" (func $c_bench_start))
            (export "bench-end"   (func $c_bench_end))
            (export "rng-seed"  (func $c_rng_seed))
            (export "rng-next"  (func $c_rng_next))
            (export "os-new"    (func $c_os_new))
            (export "os-add"    (func $c_os_add))
            (export "os-mean"   (func $c_os_mean))
            (export "os-stddev" (func $c_os_stddev))
            (export "os-min"    (func $c_os_min))
            (export "os-max"    (func $c_os_max))
            (export "getdirs"   (func $c_getdirs))
            (export "open-at"   (func $c_open))
            (export "read"      (func $c_read))
            (export "get-stdout" (func $c_get_stdout))
            (export "write"     (func $c_write))
          )
        )
      )
    )

    ;; Lift `run` to the WASI CLI `run` signature: () -> result<_, _>.
    (func (export "run") (result (result))
      (canon lift (core func $main "run"))
    )
  )

  ;; =====================================================================
  ;; Root: instantiate the sub-components and re-export `run`.
  ;; =====================================================================

  ;; The root imports the bench hooks and WASI from the host and forwards them.
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

  ;; Instantiate the resource provider and the PRNG (neither has imports).
  (instance $stats-inst (instantiate $stats))
  (instance $rng-inst (instantiate $rng))

  ;; Instantiate the runner, passing the stats and rng instances, bench, and WASI.
  (instance $runner-inst
    (instantiate $runner
      (with "online-stats" (instance $stats-inst))
      (with "rng" (instance $rng-inst))
      (with "bench" (instance $bench))
      (with "wasi:io/error@0.2.6" (instance $io-error))
      (with "wasi:io/streams@0.2.6" (instance $streams))
      (with "wasi:cli/stdout@0.2.6" (instance $stdout))
      (with "wasi:filesystem/types@0.2.6" (instance $fs-types))
      (with "wasi:filesystem/preopens@0.2.6" (instance $preopens))
    )
  )

  ;; Re-export the runner's `run` as the WASI CLI run interface.
  (alias export $runner-inst "run" (func $run))
  (instance $run-iface (export "run" (func $run)))
  (export "wasi:cli/run@0.2.6" (instance $run-iface))
)
