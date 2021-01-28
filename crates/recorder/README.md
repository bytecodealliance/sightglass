sightglass-recorder
===================

### Calibration

Is `sightglass-recorder` accurate? The mechanisms used for measuring and the API for capturing
portions of a Wasm benchmark (`bench.start`, `bench.end`) introduce overhead to our
benchmarking--but how much?

First, we examine the overhead of leaving the Wasm engine through the `bench.start` and `bench.end`
imported functions. The [noop](../../benchmarks-next/noop) benchmark calls these functions
back-to-back. We run noop using both the `wall-cycles` and `perf-counters` measurement types and 
aggregate the results:

```bash
$ cargo run -- in-process-benchmark --engine wasmtime --output-format csv --num-iterations 30 benchmarks-next/noop/benchmark.wasm --measure wall-cycles | grep Execution | cargo run -- analyze --input-format csv --output-format csv
x86_64,wasmtime,benchmarks-next/noop/benchmark.wasm,Execution,wall-cycles,680,1062,808.2,61.506666666666675

$ cargo run -- in-process-benchmark --engine wasmtime --output-format csv --num-iterations 30 benchmarks-next/noop/benchmark.wasm --measure perf-counters | grep Execution | cargo run -- analyze --input-format csv --output-format csv
x86_64,wasmtime,benchmarks-next/noop/benchmark.wasm,Execution,cache-accesses,16,186,45.0,17.866666666666667
x86_64,wasmtime,benchmarks-next/noop/benchmark.wasm,Execution,instructions-retired,470,470,470.0,0.0
x86_64,wasmtime,benchmarks-next/noop/benchmark.wasm,Execution,cache-misses,0,4,1.8,1.1733333333333333
x86_64,wasmtime,benchmarks-next/noop/benchmark.wasm,Execution,cpu-cycles,1257,3184,1614.3666666666666,248.21555555555557
```

> Note: the CSV columns are `architecture`, `engine`, `WASM benchmark`, `phase`, `event`, `min`,
> `max`, `mean`, `deviation`.

The above data indicate that the `bench.start` and `bench.end` is executing with some overhead,
somewhere between ~800±60 (measured with `RDTSC`) and ~1600±250 (measured with `perf`). The above
measurements were observed on an Intel i7-7700K on a 5.8.8 Linux kernel. The overhead is most likely
due to the instructions required to switch from executing native code inside the Wasmtime runtime to
the measurement code imported by  `bench.start` and `bench.end` PLUS any overhead that the
measurement itself may have--e.g., the reason the `perf` results are higher is likely due to the
latter.

Overestimating this overhead at 2000 CPU cycles, what effect would this have on the execution of a
1-second workload? Running the current version of shootout-ackermann only takes ~0.08s on the above
system, but even so, the 2000-cycle overhead is relatively small:

```bash
$ cargo run -- in-process-benchmark --engine wasmtime --output-format csv --num-iterations 30 benchmarks-next/shootout-ackermann/benchmark.wasm --measure wall-cycles | grep Execution | cargo run -- analyze -i csv -o csv
x86_64,wasmtime,benchmarks-next/shootout-ackermann/benchmark.wasm,Execution,wall-cycles,5072731,6466552,5795681.366666666,463938.97333333344

$ cargo run -- in-process-benchmark --engine wasmtime --output-format csv --num-iterations 30 benchmarks-next/shootout-ackermann/benchmark.wasm --measure perf-counters | grep Execution | cargo run -- analyze -i csv -o csv
x86_64,wasmtime,benchmarks-next/shootout-ackermann/benchmark.wasm,Execution,cache-accesses,4020,16565,5107.066666666667,850.8933333333331
x86_64,wasmtime,benchmarks-next/shootout-ackermann/benchmark.wasm,Execution,cache-misses,73,1573,519.9666666666667,369.68888888888904
x86_64,wasmtime,benchmarks-next/shootout-ackermann/benchmark.wasm,Execution,cpu-cycles,5423074,9778831,5581209.3,279841.4466666665
x86_64,wasmtime,benchmarks-next/shootout-ackermann/benchmark.wasm,Execution,instructions-retired,17333381,17333382,17333381.2,0.31999999955296515

```

Using `RDTSC` we measure ackermann at ~5.80m CPU cycles and using `perf` at ~5.58m CPU cycles. With
those cycle counts, our estimated 2000-cycle overhead is in the ~0.03% range. Recall that ackermann
was only running for ~0.08s; if we run benchmarks for 10x that time, any measurement overhead will
be indistinguishable from noise. Certainly 2000 cycles fits well underneath the mean deviation
observed of ~±460,000 (`RDTSC`) or ~±280,000 (`perf`) cycles of mean deviation (though that level of
deviation is slightly suspicious: has there been a context switch?).
