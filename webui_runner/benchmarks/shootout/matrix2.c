
#include <sightglass.h>

#include <assert.h>
#include <stdlib.h>

#define LENGTH 300000
#define SIZE 10

static int **
mkmatrix2(int rows, int cols)
{
    int **m = calloc(rows, sizeof(int *));
#ifndef NO_WASI_SUPPORT
    assert(m != NULL);
#endif
    int i, j, count = 1;
    for (i = 0; i < rows; i++) {
        m[i] = calloc(cols, sizeof(int));
#ifndef NO_WASI_SUPPORT
        assert(m[i] != NULL);
#endif
        for (j = 0; j < cols; j++) {
            m[i][j] = count++;
        }
    }
    return m;
}

static void
freematrix2(int rows, int **m)
{
    while (--rows > -1) {
        free(m[rows]);
    }
    free(m);
}

static int **
mmult2(int rows, int cols, int **m1, int **m2, int **m3)
{
    int i, j, k, val;

    for (i = 0; i < rows; i++) {
        for (j = 0; j < cols; j++) {
            val = 0;
            for (k = 0; k < cols; k++) {
                val += m1[i][k] * m2[k][j];
            }
            m3[i][j] = val;
        }
    }
    return m3;
}

typedef struct matrix2Ctx_ {
    int **m1, **m2, **mm;
    int   n;
    int   res;
} matrix2Ctx;

static matrix2Ctx *g_ctx = 0;

void
matrix2_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;
    (void) ctx_p;

    static matrix2Ctx ctx;
    ctx.n  = LENGTH;
    ctx.m1 = mkmatrix2(SIZE, SIZE);
    ctx.m2 = mkmatrix2(SIZE, SIZE);
    ctx.mm = mkmatrix2(SIZE, SIZE);
#ifndef NO_WASI_SUPPORT
    assert(ctx.m1 != NULL && ctx.m2 != NULL && ctx.mm != NULL);
#endif
    g_ctx = (void *) &ctx;
}

void
matrix2_teardown(void *ctx_)
{
    //matrix2Ctx *ctx = (matrix2Ctx *) ctx_;
    matrix2Ctx *ctx = (matrix2Ctx *) g_ctx;

    freematrix2(SIZE, ctx->m1);
    freematrix2(SIZE, ctx->m2);
    freematrix2(SIZE, ctx->mm);
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
matrix2_body(void *ctx_)
#else 
int
main(void)
#endif // EMBED_ENTRY
#endif // WASM_ENTRY
{
    matrix2_setup(NULL, NULL);
    matrix2Ctx *ctx = (matrix2Ctx *) g_ctx;

    int i;
    for (i = 0; i < ctx->n; i++) {
        ctx->mm = mmult2(SIZE, SIZE, ctx->m1, ctx->m2, ctx->mm);
    }
    ctx->res = ctx->mm[0][0] + ctx->mm[2][3] + ctx->mm[3][2] + ctx->mm[4][4];
    BLACK_BOX(ctx->res);

}
