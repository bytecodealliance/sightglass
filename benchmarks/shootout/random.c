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

    static RandomCtx ctx;
    ctx.ia = IA;
    ctx.ic = IC;
    ctx.im = IM;
    ctx.n  = LENGTH;

    *ctx_p = (void *) &ctx;
}

void
random_body(void *ctx_)
{
    RandomCtx *ctx = (RandomCtx *) ctx_;

    int n = ctx->n;
    while (n--) {
        gen_random(100.0, ctx->ia, ctx->ic, ctx->im);
    }
    ctx->res = gen_random(100.0, ctx->ia, ctx->ic, ctx->im);
    BLACK_BOX(ctx->res);
}
