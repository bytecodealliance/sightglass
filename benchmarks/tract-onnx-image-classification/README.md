# Image Classification Wasmtime Benchmark

A benchmark that runs an image classifier in pure Wasm. This can be used to
benchmark the performance of float heavy computations.

Note that the classifier model is not included in the repo because it is large
and is instead downloaded if needed when running the `setup.sh` script.
