# kotlin-richards

A Kotlin port of the classic Richards benchmark: a simulation of an operating
system's scheduler. This version is a Wasm component produced via Kotlin/Wasm's
`wasmWasi` target and `wasm-tools component new`.

Richards is an integer/pointer-heavy workload built around linked lists of
"packets" routed between cooperating tasks (idle, worker, handler, device). It
exercises object allocation, virtual dispatch, and pointer chasing, and here it
also exercises the WasmGC, typed-function-references, and exception-handling
proposals that Kotlin/Wasm relies on.

## Origin

The port follows the JavaScript implementation distributed with JetStream 2:

* Source: <https://browserbench.org/JetStream2.0/Octane/richards.js>
  (also published in Google's Octane benchmark suite as `richards.js`).

That JavaScript is itself a port of Martin Richards' original BCPL benchmark
(<https://www.cl.cam.ac.uk/~mr10/Bench.html>). `Richards.kt` is a faithful,
class-for-class translation of the JavaScript.

### License

The original JavaScript `richards.js` is copyright 2006-2008 the V8 project
authors and is distributed under a BSD 3-clause license (see the header of the
source linked above). This Kotlin port is made available under the same licenses
as the rest of sightglass (Apache-2.0 WITH LLVM-exception, or MIT).
