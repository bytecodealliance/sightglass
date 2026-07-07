pub mod effect_size;
pub mod keys;
pub mod report_stats;
pub mod summarize;

/// Turn a Wasm benchmark's path into a short, readable display label.
pub(crate) fn benchmark_label(wasm: &str) -> &str {
    let stripped = wasm.strip_prefix("benchmarks/").unwrap_or(wasm);
    let parts: Vec<&str> = stripped.split('/').collect();
    let last = parts.last().copied().unwrap_or(stripped);
    let stem = last.strip_suffix(".wasm").unwrap_or(last);
    if stem == "benchmark" && parts.len() >= 2 {
        parts[parts.len() - 2]
    } else {
        stem
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn benchmark_label_matches_pca_r() {
        // The file stem is kept for normally-named benchmarks.
        assert_eq!(
            benchmark_label("benchmarks/spidermonkey/spidermonkey-json.wasm"),
            "spidermonkey-json"
        );
        assert_eq!(
            benchmark_label("spidermonkey/spidermonkey-json.wasm"),
            "spidermonkey-json"
        );

        // The generic `benchmark.wasm` falls back to the parent directory.
        assert_eq!(
            benchmark_label("benchmarks/richards/benchmark.wasm"),
            "richards"
        );
        assert_eq!(benchmark_label("richards/benchmark.wasm"), "richards");

        // A leading `../../` still resolves to the parent directory.
        assert_eq!(
            benchmark_label("../../benchmarks/noop/benchmark.wasm"),
            "noop"
        );

        // Our synthetic total is preserved.
        assert_eq!(benchmark_label("Sum Total"), "Sum Total");
    }
}
