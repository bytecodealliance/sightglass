# Image Classification

This benchmark uses [wasi-nn](https://github.com/WebAssembly/wasi-nn), a proposed [WebAssembly System Interface](https://github.com/WebAssembly/WASI) API for machine
learning (ML). It uses the OpenVINO backend to identify the subject of the test image, and measures the time required for a single inference to run.

## Notes:
- You must install and setup OpenVINO to run this benchmark. Run `setup.sh` prior to running the benchmark.
- You must use `--engine-flags=--wasi-modules=experimental-wasi-nn` when running this benchmark.