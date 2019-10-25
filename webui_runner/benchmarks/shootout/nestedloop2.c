
#include <sightglass.h>

#include <stdlib.h>

#define LENGTH 30

typedef struct NestedLoop2Ctx_ {
    int n;
    int x;
    int res;
} NestedLoop2Ctx;


static NestedLoop2Ctx *g_ctx = 0;

void
nestedloop2_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;
    (void) ctx_p;

    static NestedLoop2Ctx ctx;
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
nestedloop2_body(void *ctx_)
#else 
int
main(void)
#endif // EMBED_ENTRY
#endif // WASM_ENTRY
{
    nestedloop2_setup(NULL, NULL);
    NestedLoop2Ctx *ctx =  g_ctx;

    int n = ctx->n;
    int x = ctx->x;
    int a, b, c, d, e, f;

    for (a = 0; a < n; a++) {
        for (b = 1; b < n; b++) {
            for (c = 2; c < n; c++) {
                for (d = 3; d < n; d++) {
                    for (e = 4; e < n; e++) {
                        for (f = 5; f < n; f++) {
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
