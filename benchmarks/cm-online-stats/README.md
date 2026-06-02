# `cm-online-stats`

A component-model benchmark. It exercises tiny components making many, frequent
cross-component calls.

The workload computes the running mean, standard deviation, minimum, and maximum
of a stream of pseudo-random `f64` samples using [Welford's online
algorithm][welford].

[welford]: https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm

## Structure

The benchmark is hand-written in the WebAssembly text format (`.wat`) as a root
component containing three sub-components:

1. **`$stats`** defines and exports an `online-stats` resource:

   ```wit
   resource online-stats;
   new-online-stats: func() -> own<online-stats>;
   add-sample: func(stats: borrow<online-stats>, sample: f64);
   get-mean: func(stats: borrow<online-stats>) -> f64;
   get-std-dev: func(stats: borrow<online-stats>) -> f64;
   get-min: func(stats: borrow<online-stats>) -> f64;
   get-max: func(stats: borrow<online-stats>) -> f64;
   ```

   The resource's representation is a pointer into the sub-component's linear
   memory, bump-allocated by `new-online-stats`. Statistics are updated
   incrementally on each `add-sample`.

2. **`$rng`** is a global xorshift128+ pseudo-random number generator. Its state
   lives in module globals (rather than supporting many RNGs as resources):

   ```wit
   seed: func(seed: u64);
   next-f64: func() -> f64;
   ```

3. **`$runner`** imports the `$stats` and `$rng` instances, the `bench` timing
   hooks, and the slice of WASI it needs, and exports a `run` function compatible
   with the WASI CLI `command` world.

   It:
   * reads `SEED,ITERS` from `default.input` via `wasi:filesystem`,
   * seeds the PRNG with `SEED`,
   * calls `bench.start`,
   * creates an `online-stats` resource,
   * feeds it `ITERS` random `f64` samples in `[0, 1)` drawn from `$rng`,
   * reads back the mean/std-dev/min/max,
   * calls `bench.end`,
   * and prints the statistics to stdout via `wasi:cli/stdout`.

The root component wires everything together.

## Running

```
cargo run -- benchmark \
    -e path/to/libwasmtime_bench_api.{so,dylib} \
    -- benchmarks/cm-online-stats/cm-online-stats.wasm
```

## Workload

`default.input` contains `SEED,ITERS`. `ITERS` is chosen so that the benchmark
runs in a reasonable but non-trivial amount of time.

## License

Original work, part of sightglass, available under the same licenses as the rest
of the repository (Apache-2.0 WITH LLVM-exception, or MIT).
