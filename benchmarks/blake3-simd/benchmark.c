#include "blake3.h"
#include <fcntl.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include "sightglass.h"

// The workload is read from `default.input` at runtime so it can be resized
// without recompiling (like the blake3-scalar benchmark). The C BLAKE3
// hand-written SSE2 implementation is compiled to Wasm SIMD via the SSE->Wasm
// shim in wasm_sse_compat.h.

int main()
{
    const char *path = "default.input";
    int fd = open(path, O_RDONLY);
    if (fd < 0)
    {
        fprintf(stderr, "failed to open default.input\n");
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
        ssize_t nread = read(fd, buffer + len, cap - len);
        if (nread < 0)
        {
            fprintf(stderr, "read failed\n");
            return 1;
        }
        if (nread == 0)
            break;
        len += (size_t)nread;
    }
    close(fd);

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
