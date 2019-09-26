
#include <sightglass.h>

#include <math.h>
#include <stdlib.h>

#define LENGTH 10000
#define ITERATIONS 1000

#define IM 139968
#define IA 3877
#define IC 29573

static double
gen_random(double max)
{
    static long last = 42;
    return max * (last = (last * IA + IC) % IM) / IM;
}

static void
my_heapsort(int n, double *ra)
{
    int    i, j;
    int    ir = n;
    int    l  = (n >> 1) + 1;
    double rra;

    for (;;) {
        if (l > 1) {
            rra = ra[--l];
        } else {
            rra    = ra[ir];
            ra[ir] = ra[1];
            if (--ir == 1) {
                ra[1] = rra;
                return;
            }
        }

        i = l;
        j = l << 1;
        while (j <= ir) {
            if (j < ir && ra[j] < ra[j + 1]) {
                ++j;
            }
            if (rra < ra[j]) {
                ra[i] = ra[j];
                j += (i = j);
            } else {
                j = ir + 1;
            }
        }
        ra[i] = rra;
    }
}

typedef struct HeapSortCtx_ {
    int     n;
    double *ary;
    double  res;
} HeapSortCtx;

static HeapSortCtx *g_ctx = 0;

void
heapsort_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;
    (void) ctx_p;

    static HeapSortCtx ctx;
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
heapsort_body(void *ctx_)
#else 
int
main(void)
#endif // EMBED_ENTRY
#endif // WASM_ENTRY
{
    heapsort_setup(NULL, NULL);
    HeapSortCtx *ctx = g_ctx;
    
    int i, j;
    ctx->ary = calloc(ctx->n + 1, sizeof *ctx->ary);
    for (i = 0; i < ITERATIONS; i++) {
        BLACK_BOX(ctx->ary);
        BLACK_BOX(ctx->n);
        for (j = 1; j <= ctx->n; j++) {
            ctx->ary[j] = gen_random(1.0);
        }
        my_heapsort(ctx->n, ctx->ary);
        ctx->res = ctx->ary[ctx->n];
        BLACK_BOX(ctx->res);
    }
    free(ctx->ary);
}
