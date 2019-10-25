
#include <sightglass.h>

#include <stdlib.h>

#define LENGTH 30

typedef struct NestedLoopCtx_ {
    int n;
    int res;
} NestedLoopCtx;

static NestedLoopCtx *g_ctx = 0;

void
nestedloop_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;
    (void) ctx_p;

    static NestedLoopCtx ctx;
    ctx.n = LENGTH;

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
nestedloop_body(void *ctx_)
#else 
int
main(void)
#endif // EMBED_ENTRY
#endif // WASM_ENTRY
{
    nestedloop_setup(NULL, NULL);
    NestedLoopCtx *ctx = g_ctx;

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
