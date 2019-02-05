
#include <sightglass.h>

#include <assert.h>
#include <stdlib.h>
#include <string.h>

#define STR_SIZE 500000
#define ITERATIONS 1000

typedef struct StrchrCtx_ {
    char * str;
    size_t str_size;
    size_t ret;
} StrchrCtx;

void
strchr_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;

    static StrchrCtx ctx;
    ctx.str_size = STR_SIZE;

    *ctx_p = (void *) &ctx;
}

void
strchr_body(void *ctx_)
{
    StrchrCtx *ctx = (StrchrCtx *) ctx_;

    char * str;
    size_t ret = (size_t) 0U;
    int    i;

    ctx->str = malloc(ctx->str_size);
    assert(ctx->str != NULL);
    str = ctx->str;

    assert(ctx->str_size >= 2U);
    memset(ctx->str, 'x', ctx->str_size);
    ctx->str[ctx->str_size - 1U] = 0;

    for (i = 0; i < ITERATIONS; i++) {
        BLACK_BOX(ctx->str);
        ret += (strchr(ctx->str, 'A') != NULL);
    }

    free(ctx->str);
    BLACK_BOX(ret);
    ctx->ret = ret;
}
