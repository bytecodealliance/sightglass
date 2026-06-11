//! Test `sightglass-cli pca-metrics`.

use super::util::{benchmark, sightglass_cli, test_engine};
use assert_cmd::prelude::*;
use std::path::{Path, PathBuf};

/// Recursively collect every `*.wasm` file under `dir`.
fn collect_wasms(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_wasms(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("wasm") {
            out.push(path);
        }
    }
}

/// Every `benchmarks/**/*.wasm` file, relative to this crate's directory (the
/// working directory when tests run).
fn all_benchmark_wasms() -> Vec<PathBuf> {
    let mut wasms = Vec::new();
    collect_wasms(Path::new("../../benchmarks"), &mut wasms);
    // The `image-classification` benchmark imports `wasi_ephemeral_nn`, which we
    // don't provide a host implementation for, so it can't be run here.
    wasms.retain(|p| {
        !p.components()
            .any(|c| c.as_os_str() == "image-classification")
    });
    wasms.sort();
    assert!(
        !wasms.is_empty(),
        "expected to find benchmark `.wasm` files under ../../benchmarks"
    );
    wasms
}

/// `sightglass-cli pca-metrics` should succeed on every benchmark `.wasm` we
/// ship and emit parseable CSV.
///
/// All benchmarks are passed in a single invocation so the (relatively
/// expensive) Wasmtime engine used for dynamic metrics is created once and
/// reused; the command processes the files one at a time. We cap each benchmark
/// to a small fuel budget so the test runs quickly rather than executing every
/// benchmark's full workload.
#[test]
fn pca_metrics_succeeds_on_all_benchmarks() {
    let wasms = all_benchmark_wasms();

    let assert = sightglass_cli()
        .arg("pca-metrics")
        .arg("--engine")
        .arg(test_engine())
        .arg("--fuel")
        .arg("10000")
        .args(&wasms)
        .assert()
        .success();

    // The output should parse as CSV: a `benchmark` column followed by rows
    // that all deserialize.
    let stdout = &assert.get_output().stdout;
    let mut reader = csv::Reader::from_reader(stdout.as_slice());

    let headers: Vec<String> = reader
        .headers()
        .expect("output should have a CSV header")
        .iter()
        .map(str::to_string)
        .collect();
    assert_eq!(
        headers.first().map(String::as_str),
        Some("benchmark"),
        "first CSV column should be `benchmark`"
    );

    let mut rows = 0;
    for record in reader.records() {
        let record = record.expect("every CSV row should parse");
        rows += 1;

        let mut static_sum = 0.0;
        let mut dynamic_sum = 0.0;
        for (name, field) in headers.iter().zip(record.iter()) {
            if name == "benchmark" {
                continue;
            }

            let value: f64 = field
                .parse()
                .unwrap_or_else(|_| panic!("column `{name}` should be a number, got {field:?}"));

            if name.starts_with("static_") {
                static_sum += value;
            } else if name.starts_with("dynamic_") && name != "dynamic_total_inst_count" {
                dynamic_sum += value;
            }
        }

        // The static instruction mix is a distribution over a module's
        // instructions, so its ratios sum to ~1.0.
        assert!(
            (static_sum - 1.0).abs() < 1e-6,
            "static instruction ratios should sum to ~1.0, got {static_sum}"
        );
        // The dynamic instruction mix sums to ~1.0 when the benchmark executed
        // instructions while benchmarking was active, or to ~0.0 when it ran out
        // of fuel before reaching `bench.start`.
        assert!(
            dynamic_sum.abs() < 1e-6 || (dynamic_sum - 1.0).abs() < 1e-6,
            "dynamic instruction ratios should sum to ~1.0 or ~0.0, got {dynamic_sum}"
        );
    }
    assert_eq!(rows, wasms.len(), "every wasm benchmark should have a row");
}

#[test]
fn pca_metrics_outputs_callgrind_ratio_columns() {
    let assert = sightglass_cli()
        .arg("pca-metrics")
        .arg("--engine")
        .arg(test_engine())
        .arg(benchmark("noop"))
        .assert()
        .success();

    let stdout = &assert.get_output().stdout;
    let headers: Vec<String> = csv::Reader::from_reader(stdout.as_slice())
        .headers()
        .expect("output should have a CSV header")
        .iter()
        .map(str::to_string)
        .collect();

    for name in [
        "wasm_insts_per_native_inst",
        "conditional_branch_misses",
        "conditional_branches",
        "indirect_branch_misses",
        "indirect_branches",
        "l1_dcache_read_misses",
        "l1_dcache_write_misses",
        "ll_dcache_read_misses",
        "ll_dcache_write_misses",
        "l1_icache_misses",
        "ll_icache_misses",
    ] {
        assert!(
            headers.iter().any(|header| header == name),
            "missing `{name}` column"
        );
    }
}
