#include <stdio.h>
#include "sightglass.h"

__attribute__((export_name("add")))
__attribute__((noinline))
int add(int a, int b)
{
    return a + b;
}

int main()
{
    bench_start();
    int c = add(40, 2);
    bench_end();
    printf("%d\n", c);
}
