
#include <sightglass.h>

#include <assert.h>
#include <stdlib.h>
#include <string.h>

#define STR_SIZE 10000
#define ITERATIONS 10

typedef struct MemmoveCtx_ {
    char * str;
    size_t str_size;
    char   ret;
} MemmoveCtx;

static MemmoveCtx *g_ctx;

void
memmove_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;
    (void) ctx_p;

    static MemmoveCtx ctx;
    ctx.str_size = STR_SIZE;

    g_ctx = (void *) &ctx;
}

#ifdef WASM_ENTRY
#ifdef ALT_ENTRY
void
_call(int in)
#else
void
_start(void)
#endif
#else  // WASM_ENTRY
#ifdef EMBED_ENTRY
void
memmove_body(void *ctx_)
#else 
int
main(void)
#endif // EMBED_ENTRY
#endif // WASM_ENTRY
{
    memmove_setup(NULL, NULL);
    MemmoveCtx *ctx = (MemmoveCtx *) g_ctx;
    int         i;
    size_t      j;

    ctx->str = calloc(ctx->str_size, (size_t) 1U);
#ifndef NO_WASI_SUPPORT
    assert(ctx->str != NULL);
#endif

    for (i = 0; i < ITERATIONS; i++) {
        BLACK_BOX(ctx->str);
        for (j = (size_t) 0U; j < ctx->str_size - (size_t) 1U; j++) {
            memmove(&ctx->str[j], ctx->str, ctx->str_size - j);
        }
        for (j = (size_t) 0U; j < ctx->str_size - (size_t) 1U; j++) {
            memmove(ctx->str, &ctx->str[j], ctx->str_size - j);
        }
    }

    ctx->ret = ctx->str[0];
    BLACK_BOX(ctx->str);
    free(ctx->str);
}
