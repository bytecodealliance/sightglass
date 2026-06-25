// x86 SSE/SSSE3 -> WebAssembly SIMD compatibility shim for intgemm.
//
// wasi-sdk's clang cannot compile x86 SSE intrinsics for wasm (its
// `<emmintrin.h>`/`<tmmintrin.h>` #error out, and `-msse*` are rejected), and
// intgemm's kernels are written against those intrinsics. Rather than pull in a
// whole library, this header provides exactly the `_mm_*` intrinsics intgemm
// uses, implemented with the native Wasm SIMD intrinsics from
// `<wasm_simd128.h>`. On a native (x86) target it just includes the real
// headers, so the same intgemm source builds either way.
//
// Note: __m128, __m128i and __m128d are defined as *distinct* vector types
// (different element types) on purpose. intgemm overloads functions on them
// (e.g. callbacks' run_callbacks), so they must not collapse to a single type
// the way wasm's `v128_t` would.
#pragma once

#if defined(__wasm_simd128__)

#include <wasm_simd128.h>
#include <cstdint>

typedef long long __m128i __attribute__((__vector_size__(16), __may_alias__));
typedef float __m128 __attribute__((__vector_size__(16), __may_alias__));
typedef double __m128d __attribute__((__vector_size__(16), __may_alias__));

// Reinterpret (bit-cast) helpers between our typed vectors and wasm's v128_t.
#define INTGEMM_W(x) ((v128_t)(x))

// --- Load / store / set / zero ---------------------------------------------
static inline __m128 _mm_load_ps(const float *p) { return (__m128)wasm_v128_load(p); }
static inline __m128 _mm_loadu_ps(const float *p) { return (__m128)wasm_v128_load(p); }
static inline void _mm_storeu_ps(float *p, __m128 a) { wasm_v128_store(p, INTGEMM_W(a)); }
static inline __m128i _mm_set1_epi8(int8_t v) { return (__m128i)wasm_i8x16_splat(v); }
static inline __m128i _mm_set1_epi16(int16_t v) { return (__m128i)wasm_i16x8_splat(v); }
static inline __m128i _mm_set1_epi32(int32_t v) { return (__m128i)wasm_i32x4_splat(v); }
static inline __m128 _mm_set1_ps(float v) { return (__m128)wasm_f32x4_splat(v); }
static inline __m128d _mm_set1_pd(double v) { return (__m128d)wasm_f64x2_splat(v); }
static inline __m128i _mm_setzero_si128(void) { return (__m128i)wasm_i64x2_const(0, 0); }
static inline __m128 _mm_setzero_ps(void) { return (__m128)wasm_i64x2_const(0, 0); }
static inline __m128d _mm_setzero_pd(void) { return (__m128d)wasm_i64x2_const(0, 0); }

// --- Integer arithmetic ----------------------------------------------------
static inline __m128i _mm_add_epi8(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_add(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128i _mm_add_epi16(__m128i a, __m128i b) { return (__m128i)wasm_i16x8_add(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128i _mm_add_epi32(__m128i a, __m128i b) { return (__m128i)wasm_i32x4_add(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128i _mm_sub_epi8(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_sub(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128i _mm_adds_epi16(__m128i a, __m128i b) { return (__m128i)wasm_i16x8_add_sat(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128i _mm_abs_epi8(__m128i a) { return (__m128i)wasm_i8x16_abs(INTGEMM_W(a)); }
static inline __m128i _mm_max_epi8(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_max(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128i _mm_max_epi16(__m128i a, __m128i b) { return (__m128i)wasm_i16x8_max(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128i _mm_mullo_epi16(__m128i a, __m128i b) { return (__m128i)wasm_i16x8_mul(INTGEMM_W(a), INTGEMM_W(b)); }

// High 16 bits of signed 16-bit products.
static inline __m128i _mm_mulhi_epi16(__m128i a, __m128i b) {
  v128_t lo = wasm_i32x4_extmul_low_i16x8(INTGEMM_W(a), INTGEMM_W(b));
  v128_t hi = wasm_i32x4_extmul_high_i16x8(INTGEMM_W(a), INTGEMM_W(b));
  // Pick the high 16 bits (odd 16-bit lanes) of each 32-bit product.
  return (__m128i)wasm_i16x8_shuffle(lo, hi, 1, 3, 5, 7, 9, 11, 13, 15);
}
// Unsigned 32x32 -> 64-bit products of the even 32-bit lanes (0 and 2).
static inline __m128i _mm_mul_epu32(__m128i a, __m128i b) {
  v128_t ea = wasm_i32x4_shuffle(INTGEMM_W(a), INTGEMM_W(a), 0, 2, 0, 2);
  v128_t eb = wasm_i32x4_shuffle(INTGEMM_W(b), INTGEMM_W(b), 0, 2, 0, 2);
  return (__m128i)wasm_u64x2_extmul_low_u32x4(ea, eb);
}
// Multiply adjacent unsigned*signed bytes, add pairs with signed saturation.
static inline __m128i _mm_maddubs_epi16(__m128i a, __m128i b) {
  v128_t alo = wasm_u16x8_extend_low_u8x16(INTGEMM_W(a));
  v128_t ahi = wasm_u16x8_extend_high_u8x16(INTGEMM_W(a));
  v128_t blo = wasm_i16x8_extend_low_i8x16(INTGEMM_W(b));
  v128_t bhi = wasm_i16x8_extend_high_i8x16(INTGEMM_W(b));
  v128_t plo = wasm_i16x8_mul(alo, blo); // products of bytes 0..7
  v128_t phi = wasm_i16x8_mul(ahi, bhi); // products of bytes 8..15
  v128_t even = wasm_i16x8_shuffle(plo, phi, 0, 2, 4, 6, 8, 10, 12, 14);
  v128_t odd = wasm_i16x8_shuffle(plo, phi, 1, 3, 5, 7, 9, 11, 13, 15);
  return (__m128i)wasm_i16x8_add_sat(even, odd);
}
// Multiply 16-bit pairs and add adjacent into 32-bit lanes.
static inline __m128i _mm_madd_epi16(__m128i a, __m128i b) {
  return (__m128i)wasm_i32x4_dot_i16x8(INTGEMM_W(a), INTGEMM_W(b));
}
// Negate/zero each byte of `a` according to the sign of the matching byte of `b`.
static inline __m128i _mm_sign_epi8(__m128i a, __m128i b) {
  v128_t z = wasm_i8x16_splat(0);
  v128_t neg = wasm_i8x16_neg(INTGEMM_W(a));
  v128_t ltz = wasm_i8x16_lt(INTGEMM_W(b), z);  // b < 0
  v128_t eqz = wasm_i8x16_eq(INTGEMM_W(b), z);  // b == 0
  v128_t r = wasm_v128_bitselect(neg, INTGEMM_W(a), ltz);
  return (__m128i)wasm_v128_andnot(r, eqz); // zero where b == 0
}

// --- Float / double arithmetic ---------------------------------------------
static inline __m128 _mm_add_ps(__m128 a, __m128 b) { return (__m128)wasm_f32x4_add(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128 _mm_sub_ps(__m128 a, __m128 b) { return (__m128)wasm_f32x4_sub(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128 _mm_mul_ps(__m128 a, __m128 b) { return (__m128)wasm_f32x4_mul(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128 _mm_div_ps(__m128 a, __m128 b) { return (__m128)wasm_f32x4_div(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128 _mm_min_ps(__m128 a, __m128 b) { return (__m128)wasm_f32x4_min(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128 _mm_max_ps(__m128 a, __m128 b) { return (__m128)wasm_f32x4_max(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128d _mm_add_pd(__m128d a, __m128d b) { return (__m128d)wasm_f64x2_add(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128d _mm_sub_pd(__m128d a, __m128d b) { return (__m128d)wasm_f64x2_sub(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128d _mm_mul_pd(__m128d a, __m128d b) { return (__m128d)wasm_f64x2_mul(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128d _mm_max_pd(__m128d a, __m128d b) { return (__m128d)wasm_f64x2_max(INTGEMM_W(a), INTGEMM_W(b)); }

// --- Conversions ------------------------------------------------------------
static inline __m128 _mm_cvtepi32_ps(__m128i a) { return (__m128)wasm_f32x4_convert_i32x4(INTGEMM_W(a)); }
static inline __m128i _mm_cvttps_epi32(__m128 a) { return (__m128i)wasm_i32x4_trunc_sat_f32x4(INTGEMM_W(a)); }
// Round to nearest (ties to even), then convert, matching SSE's default mode.
static inline __m128i _mm_cvtps_epi32(__m128 a) { return (__m128i)wasm_i32x4_trunc_sat_f32x4(wasm_f32x4_nearest(INTGEMM_W(a))); }
static inline __m128 _mm_castsi128_ps(__m128i a) { return (__m128)(a); }

// --- Bitwise ----------------------------------------------------------------
static inline __m128i _mm_and_si128(__m128i a, __m128i b) { return (__m128i)wasm_v128_and(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128i _mm_or_si128(__m128i a, __m128i b) { return (__m128i)wasm_v128_or(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128i _mm_xor_si128(__m128i a, __m128i b) { return (__m128i)wasm_v128_xor(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128 _mm_and_ps(__m128 a, __m128 b) { return (__m128)wasm_v128_and(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128 _mm_andnot_ps(__m128 a, __m128 b) { return (__m128)wasm_v128_andnot(INTGEMM_W(b), INTGEMM_W(a)); } // (~a) & b

// --- Comparisons (produce all-ones / all-zeros masks) ----------------------
static inline __m128i _mm_cmpeq_epi8(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_eq(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128i _mm_cmpgt_epi8(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_gt(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128i _mm_cmplt_epi8(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_lt(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128i _mm_cmpgt_epi16(__m128i a, __m128i b) { return (__m128i)wasm_i16x8_gt(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128i _mm_cmplt_epi32(__m128i a, __m128i b) { return (__m128i)wasm_i32x4_lt(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128 _mm_cmplt_ps(__m128 a, __m128 b) { return (__m128)wasm_f32x4_lt(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128 _mm_cmpneq_ps(__m128 a, __m128 b) { return (__m128)wasm_f32x4_ne(INTGEMM_W(a), INTGEMM_W(b)); }

// --- Saturating narrowing (pack) -------------------------------------------
static inline __m128i _mm_packs_epi16(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_narrow_i16x8(INTGEMM_W(a), INTGEMM_W(b)); }
static inline __m128i _mm_packs_epi32(__m128i a, __m128i b) { return (__m128i)wasm_i16x8_narrow_i32x4(INTGEMM_W(a), INTGEMM_W(b)); }

// --- Interleave (unpack) ----------------------------------------------------
static inline __m128i _mm_unpacklo_epi8(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_shuffle(INTGEMM_W(a), INTGEMM_W(b), 0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23); }
static inline __m128i _mm_unpackhi_epi8(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_shuffle(INTGEMM_W(a), INTGEMM_W(b), 8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31); }
static inline __m128i _mm_unpacklo_epi16(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_shuffle(INTGEMM_W(a), INTGEMM_W(b), 0, 1, 16, 17, 2, 3, 18, 19, 4, 5, 20, 21, 6, 7, 22, 23); }
static inline __m128i _mm_unpackhi_epi16(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_shuffle(INTGEMM_W(a), INTGEMM_W(b), 8, 9, 24, 25, 10, 11, 26, 27, 12, 13, 28, 29, 14, 15, 30, 31); }
static inline __m128i _mm_unpacklo_epi32(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_shuffle(INTGEMM_W(a), INTGEMM_W(b), 0, 1, 2, 3, 16, 17, 18, 19, 4, 5, 6, 7, 20, 21, 22, 23); }
static inline __m128i _mm_unpackhi_epi32(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_shuffle(INTGEMM_W(a), INTGEMM_W(b), 8, 9, 10, 11, 24, 25, 26, 27, 12, 13, 14, 15, 28, 29, 30, 31); }
static inline __m128i _mm_unpacklo_epi64(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_shuffle(INTGEMM_W(a), INTGEMM_W(b), 0, 1, 2, 3, 4, 5, 6, 7, 16, 17, 18, 19, 20, 21, 22, 23); }
static inline __m128i _mm_unpackhi_epi64(__m128i a, __m128i b) { return (__m128i)wasm_i8x16_shuffle(INTGEMM_W(a), INTGEMM_W(b), 8, 9, 10, 11, 12, 13, 14, 15, 24, 25, 26, 27, 28, 29, 30, 31); }

// --- Shifts by a (compile-time or runtime) count ---------------------------
static inline __m128i _mm_slli_epi16(__m128i a, int c) { return (__m128i)wasm_i16x8_shl(INTGEMM_W(a), c); }
static inline __m128i _mm_srli_epi16(__m128i a, int c) { return (__m128i)wasm_u16x8_shr(INTGEMM_W(a), c); }
static inline __m128i _mm_srai_epi16(__m128i a, int c) { return (__m128i)wasm_i16x8_shr(INTGEMM_W(a), c); }
static inline __m128i _mm_srai_epi32(__m128i a, int c) { return (__m128i)wasm_i32x4_shr(INTGEMM_W(a), c); }

// --- Shuffles (immediate); macros so the lane indices stay compile-time -----
#define _mm_shuffle_epi32(a, imm) \
  ((__m128i)wasm_i32x4_shuffle((v128_t)(a), (v128_t)(a), (imm) & 3, ((imm) >> 2) & 3, ((imm) >> 4) & 3, ((imm) >> 6) & 3))
#define _mm_shuffle_ps(a, b, imm) \
  ((__m128)wasm_i32x4_shuffle((v128_t)(a), (v128_t)(b), (imm) & 3, ((imm) >> 2) & 3, 4 + (((imm) >> 4) & 3), 4 + (((imm) >> 6) & 3)))
// Byte-wise shift right of the whole 128-bit value (shifting in zeros).
#define _mm_srli_si128(a, imm) \
  ((__m128i)wasm_i8x16_shuffle((v128_t)(a), wasm_i8x16_splat(0), \
    (imm) + 0, (imm) + 1, (imm) + 2, (imm) + 3, (imm) + 4, (imm) + 5, (imm) + 6, (imm) + 7, \
    (imm) + 8, (imm) + 9, (imm) + 10, (imm) + 11, (imm) + 12, (imm) + 13, (imm) + 14, (imm) + 15))

#undef INTGEMM_W

#else // !__wasm_simd128__ : native x86 build uses the real intrinsics.

#include <emmintrin.h>
#include <tmmintrin.h>
#include <xmmintrin.h>
#include <smmintrin.h>

#endif
