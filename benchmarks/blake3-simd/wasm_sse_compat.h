// x86 SSE2 -> WebAssembly SIMD compatibility shim for BLAKE3's blake3_sse2.c.
//
// wasi-sdk's clang cannot compile x86 SSE intrinsics for wasm (its
// `<emmintrin.h>` #errors, and `-msse2` is rejected). BLAKE3's hand-written
// SSE2 kernels are written against those intrinsics, so this header provides
// exactly the `_mm_*` intrinsics blake3_sse2.c uses, implemented with the native
// Wasm SIMD intrinsics from `<wasm_simd128.h>`. On a native (x86) build it just
// includes the real header, so the same source compiles either way.
#pragma once

#if defined(__wasm_simd128__)

#include <wasm_simd128.h>
#include <stdint.h>

typedef long long __m128i __attribute__((__vector_size__(16), __may_alias__));
typedef float __m128 __attribute__((__vector_size__(16), __may_alias__));

// --- Arithmetic / logical --------------------------------------------------
static inline __m128i _mm_add_epi32(__m128i a, __m128i b) { return (__m128i)wasm_i32x4_add((v128_t)(a), (v128_t)(b)); }
static inline __m128i _mm_sub_epi32(__m128i a, __m128i b) { return (__m128i)wasm_i32x4_sub((v128_t)(a), (v128_t)(b)); }
static inline __m128i _mm_and_si128(__m128i a, __m128i b) { return (__m128i)wasm_v128_and((v128_t)(a), (v128_t)(b)); }
static inline __m128i _mm_or_si128(__m128i a, __m128i b) { return (__m128i)wasm_v128_or((v128_t)(a), (v128_t)(b)); }
static inline __m128i _mm_xor_si128(__m128i a, __m128i b) { return (__m128i)wasm_v128_xor((v128_t)(a), (v128_t)(b)); }
static inline __m128i _mm_andnot_si128(__m128i a, __m128i b) { return (__m128i)wasm_v128_andnot((v128_t)(b), (v128_t)(a)); } // (~a) & b
static inline __m128i _mm_cmpeq_epi16(__m128i a, __m128i b) { return (__m128i)wasm_i16x8_eq((v128_t)(a), (v128_t)(b)); }
static inline __m128i _mm_cmpgt_epi32(__m128i a, __m128i b) { return (__m128i)wasm_i32x4_gt((v128_t)(a), (v128_t)(b)); }

// --- Splat / set -----------------------------------------------------------
static inline __m128i _mm_set1_epi16(short x) { return (__m128i)wasm_i16x8_splat(x); }
static inline __m128i _mm_set1_epi32(int x) { return (__m128i)wasm_i32x4_splat(x); }
// `set` is high-lane-first; `make` is low-lane-first.
#define _mm_set_epi32(e3, e2, e1, e0)  ((__m128i)wasm_i32x4_make((e0), (e1), (e2), (e3)))
#define _mm_setr_epi32(e0, e1, e2, e3) ((__m128i)wasm_i32x4_make((e0), (e1), (e2), (e3)))
#define _mm_set_epi16(e7, e6, e5, e4, e3, e2, e1, e0) \
  ((__m128i)wasm_i16x8_make((e0), (e1), (e2), (e3), (e4), (e5), (e6), (e7)))

// --- Load / store (unaligned) ----------------------------------------------
static inline __m128i _mm_loadu_si128(const __m128i *p) { return (__m128i)wasm_v128_load(p); }
static inline void _mm_storeu_si128(__m128i *p, __m128i a) { wasm_v128_store(p, (v128_t)(a)); }

// --- Reinterpret casts ------------------------------------------------------
static inline __m128 _mm_castsi128_ps(__m128i a) { return (__m128)(a); }
static inline __m128i _mm_castps_si128(__m128 a) { return (__m128i)(a); }

// --- Shifts (immediate or runtime count) -----------------------------------
static inline __m128i _mm_slli_epi32(__m128i a, int c) { return (__m128i)wasm_i32x4_shl((v128_t)(a), c); }
static inline __m128i _mm_srli_epi32(__m128i a, int c) { return (__m128i)wasm_u32x4_shr((v128_t)(a), c); }

// --- Interleave (unpack) ----------------------------------------------------
static inline __m128i _mm_unpacklo_epi32(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_shuffle((v128_t)(a), (v128_t)(b), 0, 1, 2, 3, 16, 17, 18, 19, 4, 5, 6, 7, 20, 21, 22, 23); }
static inline __m128i _mm_unpackhi_epi32(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_shuffle((v128_t)(a), (v128_t)(b), 8, 9, 10, 11, 24, 25, 26, 27, 12, 13, 14, 15, 28, 29, 30, 31); }
static inline __m128i _mm_unpacklo_epi64(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_shuffle((v128_t)(a), (v128_t)(b), 0, 1, 2, 3, 4, 5, 6, 7, 16, 17, 18, 19, 20, 21, 22, 23); }
static inline __m128i _mm_unpackhi_epi64(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_shuffle((v128_t)(a), (v128_t)(b), 8, 9, 10, 11, 12, 13, 14, 15, 24, 25, 26, 27, 28, 29, 30, 31); }

// --- Shuffles (immediate); macros so lane indices stay compile-time ---------
#define _mm_shuffle_epi32(a, imm) \
  ((__m128i)wasm_i32x4_shuffle((v128_t)(a), (v128_t)(a), (imm) & 3, ((imm) >> 2) & 3, ((imm) >> 4) & 3, ((imm) >> 6) & 3))
#define _mm_shuffle_ps(a, b, imm) \
  ((__m128)wasm_i32x4_shuffle((v128_t)(a), (v128_t)(b), (imm) & 3, ((imm) >> 2) & 3, 4 + (((imm) >> 4) & 3), 4 + (((imm) >> 6) & 3)))
#define _mm_shufflelo_epi16(a, imm) \
  ((__m128i)wasm_i16x8_shuffle((v128_t)(a), (v128_t)(a), (imm) & 3, ((imm) >> 2) & 3, ((imm) >> 4) & 3, ((imm) >> 6) & 3, 4, 5, 6, 7))
#define _mm_shufflehi_epi16(a, imm) \
  ((__m128i)wasm_i16x8_shuffle((v128_t)(a), (v128_t)(a), 0, 1, 2, 3, 4 + ((imm) & 3), 4 + (((imm) >> 2) & 3), 4 + (((imm) >> 4) & 3), 4 + (((imm) >> 6) & 3)))

// --- Misc -------------------------------------------------------------------
#define _MM_SHUFFLE(z, y, x, w) (((z) << 6) | ((y) << 4) | ((x) << 2) | (w))
#define _MM_HINT_T0 3
#define _mm_prefetch(p, hint) ((void)(p)) // no prefetch hint on wasm

#else // !__wasm_simd128__ : native x86 build uses the real intrinsics.

#include <immintrin.h>

#endif
