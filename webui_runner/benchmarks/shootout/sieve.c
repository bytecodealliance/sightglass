
#include <sightglass.h>

#include <stdlib.h>

#define LENGTH 17000

typedef struct SieveCtx_ {
    unsigned long n;
    unsigned long res;
} SieveCtx;

static SieveCtx *g_ctx;

void
sieve_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;
    (void) ctx_p;

    static SieveCtx ctx;
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
sieve_body(void *ctx_)
#else 
int
main(void)
#endif // EMBED_ENTRY
#endif // WASM_ENTRY
{
    sieve_setup(NULL, NULL);
    SieveCtx *ctx = (SieveCtx *) g_ctx;

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
