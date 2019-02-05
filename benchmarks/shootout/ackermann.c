
#include <sightglass.h>

typedef struct AckermannCtx_ {
    int M;
    int N;
    int res;
} AckermannCtx;

void
ackermann_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;

    static AckermannCtx ctx;
    ctx.M = 3;
    ctx.N = 7;

    *ctx_p = (void *) &ctx;
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

void
ackermann_body(void *ctx_)
{
    AckermannCtx *ctx = (AckermannCtx *) ctx_;

    ctx->res = ackermann(ctx->M, ctx->N);
    BLACK_BOX(ctx->res);
}
