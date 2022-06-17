# BLAKE3

[BLAKE3](https://github.com/BLAKE3-team/BLAKE3) is a cryptographic hash function. This benchmark is
built from the `blake3` Rust crate using the Rust toolchain with only scalar operations. It would
benefit from running `wasm-opt` on it since the Wasm file is very large (TODO).
