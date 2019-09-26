import test from 'ava';
import {history} from './fixtures/history';
import {
  calculate_average_slowdown_ratio,
  calculate_geometric_mean,
  convert_meta,
  extract_git_refs,
  extract_test_names,
  get_index_by_name
} from "../js/retrieval";

function firstKey(o) {
  return Object.keys(o)[0];
}

test('get_index_by_name', t => {
  const firstResults = history[firstKey(history)].results;
  t.is(get_index_by_name('wasmtime_app', firstResults), 1);
  t.is(get_index_by_name('base_native', firstResults), 0);
  t.is(get_index_by_name('unknown name', firstResults), -1);
});

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
  const firstResults = history[firstKey(history)].results;
  t.is(calculate_average_slowdown_ratio(1, 0, firstResults), 7.873731806770273);
});

test('extract_test_names', t => {
  t.deepEqual(extract_test_names(history), ['ackermann', 'base64', 'ctype', 'ed25519', 'fib2', 'gimli', 'heapsort', 'keccak', 'matrix', 'matrix2', 'memmove', 'minicsv', 'nestedloop', 'nestedloop2', 'nestedloop3', 'random', 'random2', 'ratelimit', 'seqhash', 'sieve', 'strcat', 'strcat2', 'strchr', 'strlen', 'strtok', 'switch', 'switch2', 'xblabla20', 'xchacha20']);
});

test.skip('extract_git_refs', t => {
  // TODO the data is incorrect; fix that and then fix this test
  t.deepEqual(extract_git_refs(history), ['wasmtime_app']);
});

test('convert_meta', t => {
  const firstMeta = history[firstKey(history)].meta;
  t.is(convert_meta(firstMeta).ts, 1568059309000);
  t.is(convert_meta(firstMeta).gitref, 'wasmtime_app'); // TODO eventually this should be a real gitref
});
