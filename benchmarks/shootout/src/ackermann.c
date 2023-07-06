#include <assert.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include "sightglass.h"

int ackermann(int M, int N)
{
    if (M == 0)
    {
        return N + 1;
    }
    if (N == 0)
    {
        return ackermann(M - 1, 1);
    }
    return ackermann(M - 1, ackermann(M, (N - 1)));
}

int read_int_from_file(char* path) {
    char* buf [32] = { 0 };

    int fd = open(path, O_RDONLY);
    assert(fd != -1);

    size_t m = 0, n = 0;
    do {
        m = read(fd, (void*) &buf, sizeof(buf) - n - 1);
        assert(m >= 0);
        n += m;
    } while (m > 0);
    assert(close(fd) == 0);

    buf[n] = '\0';
    return atoi(&buf);
}

int main()
{
    int M = read_int_from_file("./shootout-ackermann.m.input");
    int N = read_int_from_file("./shootout-ackermann.n.input");
    printf("[ackermann] running with M = %d and N = %d\n", M, N);

    bench_start();
    int result = ackermann(M, N);
    bench_end();

    printf("[ackermann] returned %d\n", result);
}
