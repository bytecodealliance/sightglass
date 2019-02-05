
#include <sightglass.h>

#include <assert.h>
#include <stdlib.h>
#include <string.h>

#define STR_SIZE 100000
#define ITERATIONS 10000

typedef struct StrtokCtx_ {
    char * str;
    size_t str_size;
    size_t ret;
} StrtokCtx;

void
strtok_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;

    static StrtokCtx ctx;
    ctx.str_size = STR_SIZE;

    *ctx_p = (void *) &ctx;
}

void
strtok_body(void *ctx_)
{
    StrtokCtx *ctx = (StrtokCtx *) ctx_;

    char * str;
    size_t ret = (size_t) 0U;
    int    i;

    ctx->str = malloc(ctx->str_size);
    assert(ctx->str != NULL);
    str = ctx->str;

    assert(ctx->str_size >= 3U);
    memset(ctx->str, 'x', ctx->str_size);
    ctx->str[0]                  = 'A';
    ctx->str[ctx->str_size - 1U] = 0;

    size_t str_size = ctx->str_size;
    for (i = 0; i < ITERATIONS; i++) {
        str[str_size - 3U] = 'A';
        str[str_size / 2U] = 'A';

        char *p;
        for (p = strtok(str, "A"); p != NULL; p = strtok(NULL, "A")) {
            ret += strlen(p);
        }
    }

    free(ctx->str);
    BLACK_BOX(ret);
    ctx->ret = ret;
}
