/**
 * @param {[number]} numbers - a series of numbers, e.g. [42, 523, 32, ...]
 * @return {number} the geometric mean of the series
 * @see https://gist.github.com/dherman/3d0b4733303eaf4bae5e
 */
export function calculate_geometric_mean(numbers) {
  const sum = (total, value) => total + Math.log(value);
  return Math.round(Math.exp(numbers.reduce(sum, 0) / numbers.length)); // TODO loss of precision?
}

/**
 * Calculate the slowdown ration between a target and reference implementation across multiple benchmark results
 * @param {string} target - the name of a runtime (i.e. implementation) in the benchmark results
 * @param {string} reference - the name of the reference runtime (i.e. the implementation to compare against as a base) in the benchmark results
 * @param {object} run - a run result contained within the output of sg-history, see `[top-level]/test/fixtures/stored-run.json`
 * @return {number} the average slowdown, e.g. the target is 1.7x slower than the reference
 */
export function calculate_average_slowdown_ratio(target, reference, run) {
  console.debug(`Calculating average slowdown ratio of ${target} to ${reference}`, run);
  const target_means = [];
  const reference_means = [];
  for (const benchmark of Object.keys(run.results)) {
    const benchmark_results = run.results[benchmark];
    if (!benchmark_results[target] || !benchmark_results[reference]) {
      console.warn(`Missing ${benchmark} results for either ${target} or ${reference} (timestamp: ${run.meta.timestamp}):`, run.results);
      continue; // we want to avoid including missing results in the calculation
    }
    target_means.push(benchmark_results[target].elapsed.mean);
    reference_means.push(benchmark_results[reference].elapsed.mean);
  }

  return calculate_geometric_mean(target_means) / calculate_geometric_mean(reference_means);
}

/**
 * Extracts the runtimes used for running the benchmarks (e.g. wasmtime, lucet)
 * @param {object} history - the output of sg-history, see `[top-level]/test/fixtures/history-output.json`
 * @return {[string]} a unique list of runtimes
 */
export function extract_runtimes(history) {
  // TODO generators?
  const runtimes = new Set();
  Object.values(history).map(r => Object.keys(r.meta.runtimes).map(rt => runtimes.add(rt)));
  return Array.from(runtimes);
}

/**
 * Extracts the benchmarks run (e.g. wasmtime, lucet)
 * @param {object} history - the output of sg-history, see `[top-level]/test/fixtures/history-output.json`
 * @return {[string]} a unique list of benchmarks
 */
export function extract_benchmarks(history) {
  const benchmarks = new Set();
  Object.values(history).map(r => Object.keys(r.results).map(b => benchmarks.add(b)));
  return Array.from(benchmarks);
}

/**
 * Extracts the benchmark suites used for running the benchmarks (e.g. shootout, polybench)
 * @param {object} history - the output of sg-history, see `[top-level]/test/fixtures/history-output.json`
 * @return {[string]} a unique list of suites
 */
export function extract_suites(history) {
  const suites = new Set();
  Object.values(history).map(r => suites.add(r.meta.suite));
  return Array.from(suites);
}

/**
 * Extract the results of a single runtime along with the reference runtime to compare against
 * @param {string} target - the name of the runtime results to extract
 * @param {object} run - a run result contained within the output of sg-history, see `[top-level]/test/fixtures/stored-run.json`
 * @return {{meta: *, runtime: *, results: *}|null} either the results truncated to only the target runtime and the
 * reference runtime or null if the target runtime is not found
 */
export function extract_target_runtime(target, run) {
  if (Object.keys(run.meta.runtimes).includes(target)) {
    const reference_runtime = run.meta.reference_runtime;
    let results = {};
    for(const r of Object.keys(run.results)) {
      results[r] = {};
      for (const rt of Object.keys(run.results[r])) {
        if(rt === target || rt === reference_runtime) {
          results[r][rt] = run.results[r][rt];
        }
      }
    }
    return {
      meta: run.meta,
      runtime: target,
      results: results
    };
  } else {
    return null;
  }
}

/**
 * Replace the metadata Unix timestamps in history with JS Date objects for easier processing; note that this modifies
 * the input history.
 * @param history - the output of sg-history, see `[top-level]/test/fixtures/history-output.json`
 * @return {object} the modified history (warning: this is the same reference as the input parameter, not a clone)
 */
export function fixup_unix_timestamps(history) {
  Object.values(history).map(r => r.meta.timestamp = new Date(r.meta.timestamp * 1000));
  return history;
}
