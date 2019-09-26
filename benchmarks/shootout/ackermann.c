
#include <sightglass.h>

typedef struct AckermannCtx_ {
    int M;
    int N;
    int res;
} AckermannCtx;

static AckermannCtx *g_ctx = 0;

void
ackermann_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;
    (void) ctx_p;

    static AckermannCtx ctx;
    ctx.M = 3;
    ctx.N = 7;

    g_ctx = (void *) &ctx;
}

static int
ackermann(int M, int N)
{
    if (M == 0) {
        return N + 1;
    }
    if (N == 0) {
        return ackermann(M - 1, 1);
    }
    return ackermann(M - 1, ackermann(M, (N - 1)));
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
ackermann_body(void *ctx_)
#else 
int
main(void)
#endif // EMBED_ENTRY
#endif // WASM_ENTRY
{
    ackermann_setup(NULL, NULL);
    AckermannCtx *ctx = g_ctx;
#ifndef SETUP_ONLY
    ctx->res = ackermann(ctx->M, ctx->N);
#endif // SETUP_ONLY
    /*
    // Test correctness
    // M = 3, N = 7 => 1021
    while (ctx->res == 1021) {
        (void) 0;
    }
    */
    BLACK_BOX(ctx->res);
}
