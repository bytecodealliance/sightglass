# intgemm

Tests integer matrix multiplication.

See https://github.com/kpu/intgemm

Built with wasi-sdk (clang). intgemm's kernels are written with x86 SSE/SSSE3
intrinsics, which wasi-sdk's clang cannot compile for Wasm, so `wasm_sse_compat.h`
reimplements the intrinsics intgemm uses on top of `<wasm_simd128.h>` and is
substituted for the x86 intrinsic headers at build time (see the `Dockerfile`).
`-DWASM` selects intgemm's SSSE3 kernels, since there is no runtime CPUID on Wasm.