
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

void
strlen_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;

    static StrlenCtx ctx;
    ctx.str_size = STR_SIZE;

    *ctx_p = (void *) &ctx;
}

void
strlen_body(void *ctx_)
{
    StrlenCtx *ctx = (StrlenCtx *) ctx_;

    size_t ret = (size_t) 0U;
    int    i;

    ctx->str = malloc(ctx->str_size);
    assert(ctx->str != NULL);

    assert(ctx->str_size >= 2U);
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
