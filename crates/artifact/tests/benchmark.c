#include <stdio.h>

__attribute__((export_name("add")))
__attribute__((noinline))
int add(int a, int b) {
    return a + b;
}

__attribute__((import_module("bench")))
__attribute__((import_name("start")))
void bench_start();

__attribute__((import_module("bench")))
__attribute__((import_name("end")))
void bench_end();

int main() {
    bench_start();
    int c = add(40, 2);
    bench_end();
    printf("%d\n", c);
}
