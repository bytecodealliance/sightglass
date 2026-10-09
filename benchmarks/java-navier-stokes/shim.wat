;; Provides the runtime imports that TeaVM's Wasm GC backend expects its JS
;; loader to supply. The build renames those imports' modules to "shim" and
;; resolves them against this module with `wasm-merge`.
(module
  ;; Exported as "memory" so the WASI preview1 adapter can use it too.
  (memory (export "memory") 33 32768)

  ;; TeaVM's malloc heap (unused by the benchmark) starts after the first page,
  ;; which holds TeaVM's static data and the benchmark's WASI buffers.
  (global (export "heapOffset") i32 (i32.const 0x10000))
  (global (export "maxSize") i32 (i32.const 0x7fff0000))
  (func (export "notifyHeapResized"))

  (func (export "sqrt") (param f64) (result f64)
    (f64.sqrt (local.get 0)))

  (func (export "currentTimeMillis") (result f64)
    (f64.const 0)))
