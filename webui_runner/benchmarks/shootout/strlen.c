
#include <sightglass.h>

#include <assert.h>
#include <stdlib.h>
#include <string.h>

#define STR_SIZE 500000
#define ITERATIONS 1000

typedef struct StrlenCtx_ {
    char * str;
    size_t str_size;
    size_t ret;
} StrlenCtx;

static StrlenCtx *g_ctx;

void
strlen_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;
    (void) ctx_p;

    static StrlenCtx ctx;
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
strlen_body(void *ctx_)
#else 
int
main(void)
#endif // EMBED_ENTRY
#endif // WASM_ENTRY
{
    strlen_setup(NULL, NULL);
    StrlenCtx *ctx = (StrlenCtx *) g_ctx;

    size_t ret = (size_t) 0U;
    int    i;

    ctx->str = malloc(ctx->str_size);
#ifndef NO_WASI_SUPPORT
    assert(ctx->str != NULL);
#endif
#ifndef NO_WASI_SUPPORT
    assert(ctx->str_size >= 2U);
#endif
    memset(ctx->str, 'x', ctx->str_size);
    ctx->str[ctx->str_size - 1U] = 0;

    for (i = 0; i < ITERATIONS; i++) {
        BLACK_BOX(ctx->str);
        ret += strlen(ctx->str);
    }

    free(ctx->str);
    BLACK_BOX(ret);
    ctx->ret = ret;
}
