#ifndef sightglass_H
#define sightglass_H 1

#include <stddef.h>
#include <stdint.h>

#define TEST_ABI_VERSION 1U

typedef struct TestsConfig {
    void     (*global_setup)(void **global_ctx_p);
    void     (*global_teardown)(void *global_ctx);
    uint64_t version;
} TestsConfig;

#ifndef black_box
static void _black_box(void *x) { (void) x; }
static void (* volatile black_box)(void *x) = _black_box;
#else
void black_box(void *x);
#endif
#define BLACK_BOX(X) black_box((void *) &(X))

#endif
