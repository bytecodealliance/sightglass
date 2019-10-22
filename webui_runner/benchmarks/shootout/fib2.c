
#include <sightglass.h>

typedef struct Fib2Ctx_ {
    unsigned long n;
    unsigned long res;
} Fib2Ctx;

static Fib2Ctx *g_ctx = 0;

static unsigned long
fib2(unsigned long n)
{
    if (n < 2) {
        return 1;
    }
    return fib2(n - 2) + fib2(n - 1);
}

void
fib2_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;
    (void) ctx_p;

    static Fib2Ctx ctx;
    ctx.n = 45;

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
fib2_body(void *ctx_)
#else
int
main(void)
#endif // EMBED_ENTRY
#endif // WASM_ENTRY
{
 
    fib2_setup(NULL, NULL);
    Fib2Ctx *ctx = g_ctx;
#ifndef SETUP_ONLY
    ctx->res = fib2(ctx->n);

#endif //SETUP_ONLY
    BLACK_BOX(ctx->res);
}
