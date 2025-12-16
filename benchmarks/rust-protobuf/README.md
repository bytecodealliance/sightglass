# Rust Protocol Buffers Serialization/Deserialization Benchmark

This benchmark tests Protocol Buffers encoding and decoding performance using the `prost` crate, a pure Rust implementation of Protocol Buffers.

## What it tests

The benchmark performs both deserialization (protobuf binary → Rust structs) and serialization (Rust structs → protobuf binary) on a realistic dataset representing user data with nested structures:

- 100 user records
- Each user has a profile, settings, and multiple posts
- Nested data structures including maps, repeated fields, and optional fields
- Total input size: ~1.0 MB of protobuf binary data

## Input Data

The `default.input` file contains 100 user records encoded in Protocol Buffers binary format. The protobuf schema is defined in `schema.proto`.

To regenerate the input data:
1. Run `python3 generate_input.py` to create JSON data
2. Run `cargo run --release --bin converter` to convert JSON to protobuf binary

## Implementation

Uses:
- `prost` 0.13 for Protocol Buffers encoding/decoding
- `prost-build` for compile-time protobuf code generation
- Measures full round-trip: decode protobuf binary → encode back to protobuf binary

## Notes

Protobuf binary format is more compact than JSON (~1.0 MB vs ~1.3 MB for the same data), making it useful for testing serialization performance with different data formats.
