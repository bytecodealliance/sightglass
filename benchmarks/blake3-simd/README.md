# BLAKE3

This benchmark is similar to [../blake3-scalar] and should return the same hash result, but the
build compiles BLAKE3's hand-written SSE2 implementation (`blake3_sse2.c`): `wasm_sse_compat.h`
maps its x86 SSE2 intrinsics onto Wasm SIMD (via `<wasm_simd128.h>`), and a small patch forces
BLAKE3's runtime dispatcher to select the SSE2 kernels on wasm.
