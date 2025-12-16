# Rust Compression/Decompression Benchmark

This benchmark tests compression and decompression performance for multiple algorithms commonly used in web and systems programming.

## What it tests

The benchmark performs compression and decompression using two algorithms:

1. **Gzip (Deflate)** - via `flate2` with pure Rust backend
   - Classic compression used in HTTP, gzip files, PNG images
   - Good balance of speed and compression ratio
   
2. **Brotli** - via `brotli` crate
   - Modern compression by Google, optimized for web content
   - Better compression ratios than gzip, especially for text/HTML/JSON

Each algorithm:
- Compresses the input data
- Decompresses it back
- Verifies the output matches the original

## Input Data

The `default.input` file (~1 MB) contains a mix of:
- Repeated patterns (compress very well)
- Structured JSON-like data (compresses well)
- Natural language text (compresses moderately)
- Random bytes (doesn't compress well)

This mix provides a realistic workload showing how algorithms perform on different data types.

## Implementation

Uses:
- `flate2` 1.0 with pure Rust backend for WASM compatibility
- `brotli` 7.0 for Brotli compression

## Performance Notes

Expected compression ratios on the test data:
- Gzip: ~24% (4:1 compression)
- Brotli: ~7% (14:1 compression)

Brotli achieves better compression but typically requires more CPU time.
