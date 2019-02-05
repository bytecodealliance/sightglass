
#include <sightglass.h>

#include <stdlib.h>

#define LENGTH 30

typedef struct NestedLoopCtx_ {
    int n;
    int res;
} NestedLoopCtx;

void
nestedloop_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;

    static NestedLoopCtx ctx;
    ctx.n = LENGTH;

    *ctx_p = (void *) &ctx;
}

void
nestedloop_body(void *ctx_)
{
    NestedLoopCtx *ctx = (NestedLoopCtx *) ctx_;

    int n = ctx->n;
    int a, b, c, d, e, f, x = 0;

    BLACK_BOX(x);
    for (a = 0; a < n; a++) {
        for (b = 0; b < n; b++) {
            for (c = 0; c < n; c++) {
                for (d = 0; d < n; d++) {
                    for (e = 0; e < n; e++) {
                        for (f = 0; f < n; f++) {
                            x++;
                        }
                    }
                }
            }
        }
    }

    BLACK_BOX(x);
    ctx->res = x;
}
