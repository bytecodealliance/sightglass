
#include <sightglass.h>

#include <stdlib.h>

#define LENGTH 40000000

#define IA 3877
#define IC 29573
#define IM 139968

typedef struct Random2Ctx_ {
    long   ia;
    long   ic;
    long   im;
    double res;
} Random2Ctx;

static inline double
gen_random2(double max, long ia, long ic, long im)
{
    static long last = 42;

    last = (last * ia + ic) % im;
    return max * last / im;
}

void
random2_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;

    static Random2Ctx ctx;
    ctx.ia = IA;
    ctx.ic = IC;
    ctx.im = IM;

    *ctx_p = (void *) &ctx;
}

void
random2_body(void *ctx_)
{
    Random2Ctx *ctx = (Random2Ctx *) ctx_;

    int n = LENGTH;
    while (n--) {
        gen_random2(100.0, ctx->ia, ctx->ic, ctx->im);
    }
    ctx->res = gen_random2(100.0, ctx->ia, ctx->ic, ctx->im);
    BLACK_BOX(ctx->res);
}
