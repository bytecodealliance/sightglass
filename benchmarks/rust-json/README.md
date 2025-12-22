# Rust JSON Serialization/Deserialization Benchmark

This benchmark tests JSON parsing and serialization performance using the `serde_json` crate, which is the de facto standard for JSON processing in Rust.

## What it tests

The benchmark performs both deserialization (JSON string → Rust structs) and serialization (Rust structs → JSON string) on a realistic dataset representing API response data with nested structures:

- 100 user records
- Each user has a profile, settings, and multiple posts
- Nested data structures including hashmaps, vectors, and optional fields
- Total input size: ~1.3 MB of JSON data

## Input Data

The `default.input` file contains 100 user records with:
- User metadata (id, username, email, timestamps)
- Profile information (bio, avatar, location, social links)
- User settings (theme, language, notifications, preferences)
- Multiple posts per user (5-15 posts with content, tags, likes)

The input file can be regenerated using `generate_input.py`.

## Implementation

Uses:
- `serde` 1.0 with derive macros for struct serialization
- `serde_json` 1.0 for JSON processing
- Measures full round-trip: parse JSON → serialize back to JSON
