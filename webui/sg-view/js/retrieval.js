/**
 * Helper to avoid undefined portions of the input data; roughly equivalent to _.get but lodash is not imported
 * @param fn_returning_value a function that when calls returns a value or throws an error
 * @param default_value the default value to return if fn_returning_value throws an error
 * @return {*} the value
 */
export function get_or_default(fn_returning_value, default_value) {
  if (!fn_returning_value || typeof fn_returning_value !== "function") {
    throw new Error(`Expected a function to be passed but found: ${fn_returning_value}`);
  }

  try {
    return fn_returning_value();
  } catch {
    console.warn(`Returning default value ${default_value} for ${fn_returning_value}`);
    return default_value;
  }
}

/**
 * Assumes that all entries in the history follow the same order for the target VMs
 * @param name the target name to search for
 * @param results a list of sightglass results with the structure: ["test_name", [["target1", {...data}], ["target2", {...data}], ...]]
 * @return {number} the index of the data in the results for the given target name
 */
export function get_index_by_name(name, results) {
  const target_names = new Set(); // relies on Set iterating in insertion order
  results.map(result => result[1].map(target => target_names.add(target[0]))); // TODO add some check to ensure there are no out-of-order results
  return Array.from(target_names).indexOf(name);
}

/**
 * @param numbers a series of numbers, e.g. [42, 523, 32, ...]
 * @return {number} the geometric mean of the series
 * @see https://gist.github.com/dherman/3d0b4733303eaf4bae5e
 */
export function calculate_geometric_mean(numbers) {
  const sum = (total, value) => total + Math.log(value);
  return Math.round(Math.exp(numbers.reduce(sum, 0) / numbers.length)); // TODO loss of precision?
}

/**
 * Calculate the slowdown ration between a target and reference implementation across multiple benchmark results
 * @param target_index the index in the benchmark results for a given implementation
 * @param reference_index the index in the benchmark results for a references implementation
 * @param history a Rust-like list of commits containing results with the structure: ["test_name", [["target1", {...data}], ["target2", {...data}], ...]]
 * @return {number} the average slowdown, e.g. the target is 1.7x slower than the reference
 */
export function calculate_average_slowdown_ratio(target_index, reference_index, history) {
  const target_means = [];
  const reference_means = [];
  for (const test of history) {
    if (!test[1][target_index] || !test[1][reference_index]) {
      console.warn(`Missing results for test: ${test[0]}`);
      continue; // we want to avoid including missing results in the calculation
    }

    target_means.push(test[1][target_index].mean);
    reference_means.push(test[1][reference_index].mean);
  }

  return calculate_geometric_mean(target_means) / calculate_geometric_mean(reference_means);
}

/**
 * Extracts the test names from a list of results
 * @param history a Rust-like list of commits containing results with the structure: ["test_name", [["target1", {...data}], ["target2", {...data}], ...]]
 * @return {[string]} a unique list of names
 */
export function extract_test_names(history) {
  const test_names = new Set();
  Object.values(history).map(commit => commit.results.map(results => test_names.add(results[0])));
  return Array.from(test_names);
}

/**
 * Extracts the git refs from a list of results
 * @param history a Rust-like list of commits containing results with the structure: ["test_name", [["target1", {...data}], ["target2", {...data}], ...]]
 * @return {[string]} a unique list of git refs
 */
export function extract_git_refs(history) {
  const git_refs = new Set();
  Object.values(history).map(commit => git_refs.add(commit.meta.gitref));
  return Array.from(git_refs);
}
