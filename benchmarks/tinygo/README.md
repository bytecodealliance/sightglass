# TinyGo Benchmarks

This directory contains multiple benchmarks compiled with TinyGo to WebAssembly.

TinyGo is a Go compiler designed for small places, producing compact
WebAssembly binaries suitable for resource-constrained environments.

## Benchmarks

### tinygo-json.wasm
JSON serialization and deserialization using Go's `encoding/json` package.
- Input: ~2.1MB JSON file with 100 user records
- Tests: Parse and stringify operations

### tinygo-regex.wasm
Regular expression matching using Go's `regexp` package.
- Input: ~6.5MB text corpus (same as `benchmarks/regex/default.input`)
- Tests: 3 regex patterns (emails, URIs, IPs) matching the Rust benchmark

## Building

From the `benchmarks` directory:

```bash
./build.sh tinygo/
```

## Adding New Benchmarks

To add a new TinyGo benchmark, create a new directory containing `main.go`,
`go.mod`, `default.input`, and expected output files. The build script will
automatically discover and build it.
