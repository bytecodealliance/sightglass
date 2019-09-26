
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

static Random2Ctx *g_ctx = 0;

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
    (void) ctx_p;
    static Random2Ctx ctx;
    ctx.ia = IA;
    ctx.ic = IC;
    ctx.im = IM;

    // Not creating heap in iwasm
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
random2_body(void *ctx_)
#else 
int
main(void)
#endif // EMBED_ENTRY
#endif // WASM_ENTRY
{
    random2_setup(NULL, NULL);
    Random2Ctx *ctx = g_ctx;

    int n = LENGTH;
    while (n--) {
        gen_random2(100.0, ctx->ia, ctx->ic, ctx->im);
    }
    ctx->res = gen_random2(100.0, ctx->ia, ctx->ic, ctx->im);
    BLACK_BOX(ctx->res);
}
