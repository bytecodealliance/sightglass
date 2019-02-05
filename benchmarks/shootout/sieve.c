
#include <sightglass.h>

#include <stdlib.h>

#define LENGTH 17000

typedef struct SieveCtx_ {
    unsigned long n;
    unsigned long res;
} SieveCtx;

void
sieve_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;

    static SieveCtx ctx;
    ctx.n = LENGTH;

    *ctx_p = (void *) &ctx;
}

void
sieve_body(void *ctx_)
{
    SieveCtx *ctx = (SieveCtx *) ctx_;

    static char flags[8192 + 1];
    int         n = ctx->n;
    long        i, k;
    int         count;

    ctx->res = 0;
    while (n--) {
        count = 0;
        for (i = 2; i <= 8192; i++) {
            flags[i] = 1;
        }
        for (i = 2; i <= 8192; i++) {
            if (flags[i]) {
                /* remove all multiples of prime: i */
                for (k = i + i; k <= 8192; k += i) {
                    flags[k] = 0;
                }
                count++;
            }
        }
        ctx->res += count;
    }
    BLACK_BOX(ctx->res);
}
