#include "blake3.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "sightglass.h"

// This benchmark is built with Emscripten (for the SSE2 -> Wasm SIMD blake3
// implementation), whose libc filesystem does not reach WASI preopened
// directories. So, like the splay benchmark, we read the workload from disk by
// calling the WASI `path_open`/`fd_read` syscalls directly. This lets the input
// be resized without recompiling, similar to the blake3-scalar benchmark.
#define WASI_IMPORT(name) \
    __attribute__((import_module("wasi_snapshot_preview1"), import_name(name)))

typedef struct {
    const void *buf;
    size_t len;
} wasi_iovec_t;

WASI_IMPORT("path_open")
int wasi_path_open(int fd, int dirflags, const char *path, size_t path_len,
                   int oflags, uint64_t rights_base, uint64_t rights_inheriting,
                   int fdflags, int *opened_fd);

WASI_IMPORT("fd_read")
int wasi_fd_read(int fd, const wasi_iovec_t *iovs, size_t iovs_len, size_t *nread);

// The benchmark directory is preopened by the runner as the first preopen (fd 3).
#define PREOPEN_FD 3
#define RIGHT_FD_READ (1ULL << 1)
#define RIGHT_FD_SEEK (1ULL << 2)

int main()
{
    const char *path = "default.input";
    int fd = -1;
    int rc = wasi_path_open(PREOPEN_FD, 0, path, strlen(path), 0,
                            RIGHT_FD_READ | RIGHT_FD_SEEK,
                            RIGHT_FD_READ | RIGHT_FD_SEEK, 0, &fd);
    if (rc != 0 || fd < 0)
    {
        fprintf(stderr, "failed to open default.input (rc=%d)\n", rc);
        return 1;
    }

    // Read the whole file into a growable buffer.
    size_t cap = 1 << 20, len = 0;
    unsigned char *buffer = (unsigned char *)malloc(cap);
    for (;;)
    {
        if (len == cap)
        {
            cap *= 2;
            buffer = (unsigned char *)realloc(buffer, cap);
        }
        wasi_iovec_t iov = {buffer + len, cap - len};
        size_t nread = 0;
        rc = wasi_fd_read(fd, &iov, 1, &nread);
        if (rc != 0)
        {
            fprintf(stderr, "fd_read failed (rc=%d)\n", rc);
            return 1;
        }
        if (nread == 0)
            break;
        len += nread;
    }

    fprintf(stderr, "[blake3] hashing ./default.input\n");
    fprintf(stderr, "[blake3] input size = %zu\n", len);

    // Initialize the hasher.
    blake3_hasher hasher;
    blake3_hasher_init(&hasher);

    // Define the hash output; BLAKE3_OUT_LEN is the default output length, 32 bytes.
    uint8_t output[BLAKE3_OUT_LEN];

    bench_start();
    blake3_hasher_update(&hasher, buffer, len);
    blake3_hasher_finalize(&hasher, output, BLAKE3_OUT_LEN);
    bench_end();

    // Print the hash as hexadecimal.
    fprintf(stderr, "[blake3] returned ");
    for (size_t i = 0; i < BLAKE3_OUT_LEN; i++)
    {
        fprintf(stderr, "%02x", output[i]);
    }
    fprintf(stderr, "\n");

    free(buffer);
    return 0;
}
