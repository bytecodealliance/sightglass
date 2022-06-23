#include "blake3.h"
#include <stdio.h>
#include <unistd.h>
#include "sightglass.h"
#define BUFFER_SIZE 65536

int main()
{
    // Initialize the hasher.
    blake3_hasher hasher;
    blake3_hasher_init(&hasher);

    // Initialize the 64K buffer.
    unsigned char buffer[BUFFER_SIZE] = { 0 };

    // Define the hash output; BLAKE3_OUT_LEN is the default output length, 32 bytes.
    uint8_t output[BLAKE3_OUT_LEN];
    fprintf(stderr, "[blake3] hashing a zero-filled buffer of %d bytes\n", BUFFER_SIZE);

    bench_start();
    blake3_hasher_update(&hasher, buffer, BUFFER_SIZE);
    blake3_hasher_finalize(&hasher, output, BLAKE3_OUT_LEN);
    bench_end();

    // Print the hash as hexadecimal.
    fprintf(stderr, "[blake3] returned ");
    for (size_t i = 0; i < BLAKE3_OUT_LEN; i++)
    {
        fprintf(stderr, "%02x", output[i]);
    }
    fprintf(stderr, "\n");

    return 0;
}
