import test from 'ava';
import {history} from '../../../test/fixtures/history-output';
import {
  calculate_average_slowdown_ratio,
  calculate_geometric_mean,
  extract_benchmarks,
  extract_runtimes,
  extract_suites,
  extract_target_runtime,
  fixup_unix_timestamps
} from "../js/retrieval";

function first_key(o) {
  return Object.keys(o)[0];
}

function deep_clone(o) {
  return JSON.parse(JSON.stringify(o));
}

test('calculate_geometric_mean', t => {
  t.is(calculate_geometric_mean([0]), 0);
  t.is(calculate_geometric_mean([0, 1]), 0); // rounded down
  t.is(calculate_geometric_mean([1, 3]), 2);
  t.is(calculate_geometric_mean([1, 3, 15]), 4);
  t.is(calculate_geometric_mean([1, 3, 9]), 3);
  t.is(calculate_geometric_mean([5]), 5);
  t.is(calculate_geometric_mean([5, 5, 5, 5]), 5);
});

test('calculate_average_slowdown_ratio', t => {
  const firstRun = history[first_key(history)];
  t.is(calculate_average_slowdown_ratio('wasmtime', 'native', firstRun), 7.873731806770273);
});

test('extract_runtimes', t => {
  t.deepEqual(extract_runtimes(history), ['wasmtime', 'native']);
});

test('extract_suites', t => {
  t.deepEqual(extract_suites(history), ['shootout']);
});

test('extract_benchmarks', t => {
  t.deepEqual(extract_benchmarks(history), ['ackermann', 'base64', 'ctype', 'ed25519', 'fib2', 'gimli', 'heapsort', 'keccak', 'matrix', 'matrix2', 'memmove', 'minicsv', 'nestedloop', 'nestedloop2', 'nestedloop3', 'random', 'random2', 'ratelimit', 'seqhash', 'sieve', 'strcat', 'strcat2', 'strchr', 'strlen', 'strtok', 'switch', 'switch2', 'xblabla20', 'xchacha20']);
});

test('extract_target_runtime', t => {
  const first_run = history[first_key(history)];
  t.is(extract_target_runtime('some-runtime', first_run), null);

  const wasmtime_results = extract_target_runtime('wasmtime', first_run);
  t.deepEqual(Object.keys(wasmtime_results.results.gimli), ['native', 'wasmtime']);
  t.is(wasmtime_results.runtime, 'wasmtime');

  const native_results = extract_target_runtime('native', first_run);
  t.deepEqual(Object.keys(native_results.results.gimli), ['native']);
  t.is(native_results.runtime, 'native');
});

test('fixup_unix_timestamps', t => {
  let cloned_history = deep_clone(history); // since fixup_unix_timestamps is destructive
  const first_run = cloned_history[first_key(cloned_history)];
  t.false(first_run.meta.timestamp instanceof Date);
  fixup_unix_timestamps(cloned_history);
  t.true(first_run.meta.timestamp instanceof Date);
});
