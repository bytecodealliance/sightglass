#include <sightglass.h>

#include <stdlib.h>

#define LENGTH 40000000

#define IA 3877
#define IC 29573
#define IM 139968

typedef struct RandomCtx_ {
    long   ia;
    long   ic;
    long   im;
    int    n;
    double res;
} RandomCtx;

static RandomCtx *g_ctx = 0;

static inline double
gen_random(double max, long ia, long ic, long im)
{
    static long last = 42;

    last = (last * ia + ic) % im;
    return max * last / im;
}

void
random_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;
    (void) ctx_p;

    static RandomCtx ctx;
    ctx.ia = IA;
    ctx.ic = IC;
    ctx.im = IM;
    ctx.n  = LENGTH;

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
random_body(void *ctx_)
#else 
int
main(void)
#endif // EMBED_ENTRY
#endif // WASM_ENTRY
{
    random_setup(NULL, NULL);
    RandomCtx *ctx = g_ctx;

    int n = ctx->n;
    while (n--) {
        gen_random(100.0, ctx->ia, ctx->ic, ctx->im);
    }
    ctx->res = gen_random(100.0, ctx->ia, ctx->ic, ctx->im);
    BLACK_BOX(ctx->res);
}
