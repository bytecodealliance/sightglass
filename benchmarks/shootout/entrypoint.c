
#include <sightglass.h>

extern TestsConfig tests_config;

#ifdef USE_LEND

#ifndef LEND_POOL_SIZE
#define LEND_POOL_SIZE (16U * 64U * (size_t) 1024U)
#endif

#undef malloc
#undef calloc
#undef free

#include <assert.h>
#include <stdlib.h>

void lend_give(void *area, size_t size);

void
global_setup(void **lend_pool_p)
{
    void *lend_pool = calloc(LEND_POOL_SIZE, (size_t) 1U);
    assert(lend_pool != NULL);
    lend_give(lend_pool, LEND_POOL_SIZE);
    *lend_pool_p = lend_pool;
}

void
global_teardown(void *lend_pool)
{
    free(lend_pool);
}

TestsConfig tests_config = { .global_setup    = global_setup,
                             .global_teardown = global_teardown,
                             .version         = TEST_ABI_VERSION };

#else // !USE_LEND

TestsConfig tests_config = { .global_setup    = NULL,
                             .global_teardown = NULL,
                             .version         = TEST_ABI_VERSION };

#endif
