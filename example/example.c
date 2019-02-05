
#include <unistd.h>

#include "../include/sightglass.h"

void
sleep_body_1(void *ctx)
{
    sleep(1);
}

void
sleep_body_2(void *ctx)
{
    sleep(2);
}

void
nop_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;
    (void) ctx_p;
}

int
nop_body(void *ctx)
{
    (void) ctx;
    return 0;
}

void
nop_teardown(void *ctx)
{
    (void) ctx;
}

void
square_setup(void *global_ctx, void **ctx_p)
{
    (void) global_ctx;
    static int x;
    *ctx_p = (void *) &x;
}

void
square_body(void *ctx)
{
    int x = *(int *) ctx;
    x *= x;
    *(int *) ctx = x;
}

void
square_body_2(void *ctx)
{
    (void) ctx;
}

void
square_body_3(void *ctx)
{
    (void) ctx;
}

extern TestsConfig tests_config;

TestsConfig tests_config = { .global_setup    = NULL,
                             .global_teardown = NULL,
                             .version         = TEST_ABI_VERSION };
