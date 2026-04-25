# Splay

A splay tree benchmark, ported to WebAssembly text format (WAT) using Wasm GC
structs and arrays. It stresses the performance of the garbage collector.

## Origin

The original JavaScript source is from [JetStream's Octane/splay.js][source]; a
similar version was previously published in Octane version 2, and it originally
came from the V8 JavaScript engine project.

[source]: https://github.com/WebKit/JetStream/blob/main/Octane/splay.js

## Running

Requires that `-Wgc -Wfunction-references` be passed to the underlying engine:

```
cargo run -- benchmark \
    -e path/to/libwasmtime_bench_api.{so,dylib} \
    --engine-flags="-Wgc -Wfunction-references" \
    -- benchmarks/splay/splay.wasm
```

## License

The original JavaScript source (`splay.js`) is copyright 2009 the V8 project
authors and copyright 2015 Apple Inc., licensed under a BSD 3-clause license.
See the header of `splay.js` for the full license text.
