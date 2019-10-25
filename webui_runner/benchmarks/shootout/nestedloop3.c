
#include <sightglass.h>

#include <stdlib.h>

#define LENGTH 30

typedef struct NestedLoop3Ctx_ {
    int n;
    int x;
    int res;
} NestedLoop3Ctx;

static NestedLoop3Ctx *g_ctx = 0;

void
nestedloop3_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;
    (void) ctx_p;

    static NestedLoop3Ctx ctx;
    ctx.n = LENGTH;
    ctx.x = 0;

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
nestedloop3_body(void *ctx_)
#else 
int
main(void)
#endif // EMBED_ENTRY
#endif // WASM_ENTRY
{
    nestedloop3_setup(NULL, NULL);
    NestedLoop3Ctx *ctx = g_ctx;

    int n = ctx->n;
    int x = ctx->x;
    int a, b, c, d, e, f;

    for (a = 0; a < n; a++) {
        for (b = a; b < n; b++) {
            for (c = b; c < n; c++) {
                for (d = c; d < n; d++) {
                    for (e = d; e < n; e++) {
                        for (f = e; f < n; f++) {
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
