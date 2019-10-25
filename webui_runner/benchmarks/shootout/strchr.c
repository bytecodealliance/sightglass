
#include <sightglass.h>

#include <assert.h>
//#include <stdlib.h>
#include <string.h>


#define STR_SIZE 500000
#define ITERATIONS 1000

//extern void *malloc(size_t size);


void* __attribute__((import_module("env"), import_name("_malloc"))) malloc(size_t);
void* __attribute__((import_module("env"), import_name("memset"))) memset(void *s, int c, size_t n);
void __attribute__((import_module("free"), import_name("free"))) free(void *);


typedef struct StrchrCtx_ {
    char * str;
    size_t str_size;
    size_t ret;
} StrchrCtx;

static StrchrCtx *g_ctx;

void
strchr_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;
    (void) ctx_p;

    static StrchrCtx ctx;
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
strchr_body(void *ctx_)
#else 
int
main(void)
#endif // EMBED_ENTRY
#endif // WASM_ENTRY
{
    strchr_setup(NULL, NULL);
    StrchrCtx *ctx = (StrchrCtx *) g_ctx;

    char * str;
    size_t ret = (size_t) 0U;
    int    i;

    ctx->str = malloc(ctx->str_size);
#ifndef NO_WASI_SUPPORT
    assert(ctx->str != NULL);
#endif
    str = ctx->str;
#ifndef NO_WASI_SUPPORT
    assert(ctx->str_size >= 2U);
#endif
    memset(str, 'x', ctx->str_size);
    str[ctx->str_size - 1U] = 0;

    for (i = 0; i < ITERATIONS; i++) {
        BLACK_BOX(str);
        ret += (strchr(str, 'A') != NULL);
    }

    free(str);
    BLACK_BOX(ret);
    ctx->ret = ret;
}
