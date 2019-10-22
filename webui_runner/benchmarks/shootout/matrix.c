
#include <sightglass.h>

#include <assert.h>
#include <stdlib.h>

#define LENGTH 300000
#define SIZE 10

static int **
mkmatrix(int rows, int cols)
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
freematrix(int rows, int **m)
{
    while (--rows > -1) {
        free(m[rows]);
    }
    free(m);
}

static int **
mmult(int rows, int cols, int **m1, int **m2, int **m3)
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

typedef struct MatrixCtx_ {
    int **m1, **m2, **mm;
    int   rows;
    int   cols;
    int   n;
    int   res;
} MatrixCtx;

static MatrixCtx *g_ctx = 0;

void
matrix_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;
    (void) ctx_p;

    static MatrixCtx ctx;
    ctx.rows = SIZE;
    ctx.cols = SIZE;
    ctx.n    = LENGTH;
    ctx.m1   = mkmatrix(ctx.rows, ctx.cols);
    ctx.m2   = mkmatrix(ctx.rows, ctx.cols);
    ctx.mm   = mkmatrix(ctx.rows, ctx.cols);
#ifndef NO_WASI_SUPPORT
    assert(ctx.m1 != NULL && ctx.m2 != NULL && ctx.mm != NULL);
#endif
    g_ctx = (void *) &ctx;
}

void
matrix_teardown(void *ctx_)
{
    //MatrixCtx *ctx = (MatrixCtx *) ctx_;
    MatrixCtx *ctx = (MatrixCtx *) g_ctx;

    freematrix(ctx->rows, ctx->m1);
    freematrix(ctx->rows, ctx->m2);
    freematrix(ctx->rows, ctx->mm);
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
matrix_body(void *ctx_)
#else 
int
main(void)
#endif // EMBED_ENTRY
#endif // WASM_ENTRY
{
    matrix_setup(NULL, NULL);
    MatrixCtx *ctx = (MatrixCtx *) g_ctx;
    int i;
    for (i = 0; i < ctx->n; i++) {
        ctx->mm = mmult(ctx->rows, ctx->cols, ctx->m1, ctx->m2, ctx->mm);
    }
    ctx->res = ctx->mm[0][0] + ctx->mm[2][3] + ctx->mm[3][2] + ctx->mm[4][4];
    BLACK_BOX(ctx->res);
}
