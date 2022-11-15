# Image Classification

This benchmark uses [wasi-nn](https://github.com/WebAssembly/wasi-nn), a proposed [WebAssembly System Interface](https://github.com/WebAssembly/WASI) API for machine
learning (ML). It uses the OpenVINO backend to identify the subject of the test image, and measures the time required for a single inference to run.

## Steps to run this benchmark from the top sightglass directory
- `./benchmarks/image-classification/setup.sh`
- `source benchmarks/image-classification/openvino/setupvars.sh`
- `cargo run -- benchmark --engine engines/wasmtime/libengine.so --engine-flags=--wasi-modules=experimental-wasi-nn -- benchmarks/image-classification/image-classification-benchmark.wasm`
## Notes:
- You must install and setup OpenVINO to run this benchmark. You can install OpenVINO and the other required files by running `setup.sh`. You'll also need to call `source benchmarks/image-classification/openvino/setupvars.sh` to set up the OpenVINO enviromnent variables each time you start a new terminal.
- You must use `--engine-flags=--wasi-modules=experimental-wasi-nn` when running this benchmark. Otherwise Wasmtime won't link the required wasi-nn functions.