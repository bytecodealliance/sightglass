;; OCaml `external` primitives forwarding to sightglass's timing hooks.
(module
  (import "bench" "start" (func $start))
  (import "bench" "end" (func $end))
  (func (export "bench_start") (param (ref eq)) (result (ref eq))
    (call $start)
    (ref.i31 (i32.const 0)))
  (func (export "bench_end") (param (ref eq)) (result (ref eq))
    (call $end)
    (ref.i31 (i32.const 0))))
