;; Implements the host imports of a Hoot standalone module on top of WASI
;; preview1, replacing Hoot's JavaScript runtime (reflect.js).
;;
;; Hoot hands host strings across the boundary as `externref`s produced by
;; `rt.wtf8_to_string`. Here a "host string" is just the guest's own WTF-8
;; byte array, externalized. File handles are externalized i31 fds.
(module
  (type $wtf8 (array (mut i8)))

  (import "wasi_snapshot_preview1" "fd_write"
    (func $fd_write (param i32 i32 i32 i32) (result i32)))
  (import "wasi_snapshot_preview1" "fd_read"
    (func $fd_read (param i32 i32 i32 i32) (result i32)))
  (import "wasi_snapshot_preview1" "fd_close"
    (func $fd_close (param i32) (result i32)))
  (import "wasi_snapshot_preview1" "path_open"
    (func $path_open (param i32 i32 i32 i32 i32 i64 i64 i32 i32) (result i32)))
  (import "wasi_snapshot_preview1" "proc_exit"
    (func $proc_exit (param i32)))

  ;; Layout: 0..16 iovec and result scratch, 64..4096 path scratch,
  ;; 4096..65536 I/O buffer.
  (memory (export "memory") 1)
  (global $buf i32 (i32.const 4096))
  (global $buf-size i32 (i32.const 61440))
  (global $preopen-fd i32 (i32.const 3))

  (func $str (param $s (ref extern)) (result (ref $wtf8))
    (ref.cast (ref $wtf8) (any.convert_extern (local.get $s))))

  ;; Copies `len` bytes of `s` starting at `start` into linear memory at `dst`.
  (func $copy-out (param $s (ref $wtf8)) (param $start i32) (param $len i32)
                  (param $dst i32)
    (local $i i32)
    (block $done
      (loop $loop
        (br_if $done (i32.ge_u (local.get $i) (local.get $len)))
        (i32.store8 (i32.add (local.get $dst) (local.get $i))
          (array.get_u $wtf8 (local.get $s)
            (i32.add (local.get $start) (local.get $i))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop))))

  ;; Writes all of `s` to `fd`, one buffer-sized chunk at a time.
  (func $write-all (param $fd i32) (param $s (ref $wtf8))
    (local $pos i32) (local $n i32) (local $len i32)
    (local.set $len (array.len (local.get $s)))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_u (local.get $pos) (local.get $len)))
        (local.set $n (i32.sub (local.get $len) (local.get $pos)))
        (if (i32.gt_u (local.get $n) (global.get $buf-size))
          (then (local.set $n (global.get $buf-size))))
        (call $copy-out (local.get $s) (local.get $pos) (local.get $n)
              (global.get $buf))
        (i32.store (i32.const 0) (global.get $buf))
        (i32.store (i32.const 4) (local.get $n))
        (if (call $fd_write (local.get $fd) (i32.const 0) (i32.const 1)
                  (i32.const 8))
          (then (unreachable)))
        (local.set $pos (i32.add (local.get $pos) (i32.load (i32.const 8))))
        (br $loop))))

  ;; rt

  (func (export "string_to_wtf8") (param $s (ref extern)) (result (ref $wtf8))
    (call $str (local.get $s)))
  (func (export "wtf8_to_string") (param $s (ref null $wtf8)) (result (ref extern))
    (extern.convert_any (ref.as_non_null (local.get $s))))

  ;; ASCII-only, which is all the reader needs.
  (func (export "string_downcase") (param $s (ref extern)) (result (ref extern))
    (local $src (ref $wtf8)) (local $dst (ref $wtf8)) (local $i i32) (local $c i32)
    (local.set $src (call $str (local.get $s)))
    (local.set $dst (array.new_default $wtf8 (array.len (local.get $src))))
    (block $done
      (loop $loop
        (br_if $done (i32.ge_u (local.get $i) (array.len (local.get $src))))
        (local.set $c (array.get_u $wtf8 (local.get $src) (local.get $i)))
        (if (i32.lt_u (i32.sub (local.get $c) (i32.const 65)) (i32.const 26))
          (then (local.set $c (i32.add (local.get $c) (i32.const 32)))))
        (array.set $wtf8 (local.get $dst) (local.get $i) (local.get $c))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop)))
    (extern.convert_any (local.get $dst)))

  (func (export "die") (param $msg (ref extern)) (param (ref eq))
    (call $write-all (i32.const 2) (call $str (local.get $msg)))
    (unreachable))
  (func (export "quit") (param $code i32)
    (call $proc_exit (local.get $code)))

  ;; debug: no introspection metadata.

  (func (export "code_name") (param (ref func)) (result externref)
    (ref.null extern))
  (func (export "code_source") (param (ref func)) (result externref i32 i32)
    (ref.null extern) (i32.const 0) (i32.const 0))

  ;; io

  (func (export "write_stdout") (param $s (ref extern))
    (call $write-all (i32.const 1) (call $str (local.get $s))))
  (func (export "write_stderr") (param $s (ref extern))
    (call $write-all (i32.const 2) (call $str (local.get $s))))
  (func (export "read_stdin") (result (ref extern))
    (extern.convert_any (array.new_default $wtf8 (i32.const 0))))

  (func $fd (param $h (ref extern)) (result i32)
    (i31.get_u (ref.cast (ref i31) (any.convert_extern (local.get $h)))))

  (func (export "open_input_file") (param $name (ref extern)) (result externref)
    (local $s (ref $wtf8))
    (local.set $s (call $str (local.get $name)))
    (if (i32.gt_u (array.len (local.get $s)) (i32.const 4000))
      (then (return (ref.null extern))))
    (call $copy-out (local.get $s) (i32.const 0) (array.len (local.get $s))
          (i32.const 64))
    ;; rights: fd_read | fd_seek | fd_tell
    (if (call $path_open (global.get $preopen-fd) (i32.const 0)
              (i32.const 64) (array.len (local.get $s))
              (i32.const 0) (i64.const 38) (i64.const 0) (i32.const 0)
              (i32.const 8))
      (then (return (ref.null extern))))
    (extern.convert_any (ref.i31 (i32.load (i32.const 8)))))

  (func (export "read_file") (param $h (ref extern)) (param $count i32) (result i32)
    (if (i32.gt_u (local.get $count) (global.get $buf-size))
      (then (local.set $count (global.get $buf-size))))
    (i32.store (i32.const 0) (global.get $buf))
    (i32.store (i32.const 4) (local.get $count))
    (if (call $fd_read (call $fd (local.get $h)) (i32.const 0) (i32.const 1)
              (i32.const 8))
      (then (unreachable)))
    (i32.load (i32.const 8)))

  (func (export "file_buffer_ref") (param (ref extern)) (param $i i32) (result i32)
    (i32.load8_u (i32.add (global.get $buf) (local.get $i))))

  (func (export "close_file") (param $h (ref extern))
    (drop (call $fd_close (call $fd (local.get $h)))))

  (func (export "file_random_access") (param (ref extern)) (result i32)
    (i32.const 0))
  (func (export "seek_file") (param (ref extern)) (param i32) (param i32) (result i32)
    (i32.const -1))
)
