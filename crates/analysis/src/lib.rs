pub mod effect_size;
pub mod keys;
pub mod report_stats;
pub mod summarize;

use sightglass_data::Summary;
use std::io::{self, Write};

/// The column headers for the min/max/mean/median/engine statistics table shared
/// by [`summarize::write`] and [`effect_size::write`].
pub(crate) const STATS_HEADERS: [&str; 5] = ["Min", "Max", "Mean", "Median", "Engine"];

/// Build a statistics-table row (matching [`STATS_HEADERS`]) from a [`Summary`]
/// and an engine label.
pub(crate) fn summary_row(summary: &Summary<'_>, engine: String) -> Vec<String> {
    vec![
        summary.min.to_string(),
        summary.max.to_string(),
        format!("{:.2}", summary.mean),
        summary.median.to_string(),
        engine,
    ]
}

/// Render `rows` as a table drawn with Unicode box-drawing characters, prefixing
/// every line with `indent`.
///
/// Each column is as wide as its widest cell (including the `headers`), cells are
/// left-aligned, and every row -- header and data alike -- is separated by a
/// horizontal rule:
///
/// ```text
///     ┌─────┬─────┬──────┬────────┬────────┐
///     │ Min │ Max │ Mean │ Median │ Engine │
///     ├─────┼─────┼──────┼────────┼────────┤
///     │ ... │ ... │ ...  │ ...    │ foo    │
///     ├─────┼─────┼──────┼────────┼────────┤
///     │ ... │ ... │ ...  │ ...    │ bar    │
///     └─────┴─────┴──────┴────────┴────────┘
/// ```
pub(crate) fn write_table(
    output: &mut dyn Write,
    indent: &str,
    headers: &[&str],
    rows: &[Vec<String>],
) -> io::Result<()> {
    let mut widths: Vec<usize> = headers.iter().map(|h| h.chars().count()).collect();
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            widths[i] = widths[i].max(cell.chars().count());
        }
    }

    fn rule(
        output: &mut dyn Write,
        indent: &str,
        widths: &[usize],
        left: char,
        mid: char,
        right: char,
    ) -> io::Result<()> {
        write!(output, "{indent}{left}")?;
        for (i, w) in widths.iter().enumerate() {
            write!(output, "{}", "─".repeat(w + 2))?;
            write!(
                output,
                "{}",
                if i + 1 == widths.len() { right } else { mid }
            )?;
        }
        writeln!(output)
    }

    fn row(
        output: &mut dyn Write,
        indent: &str,
        widths: &[usize],
        cells: &[String],
    ) -> io::Result<()> {
        write!(output, "{indent}│")?;
        for (cell, width) in cells.iter().zip(widths) {
            write!(output, " {cell:<width$} │")?;
        }
        writeln!(output)
    }

    rule(output, indent, &widths, '┌', '┬', '┐')?;
    let header_cells: Vec<String> = headers.iter().map(|h| h.to_string()).collect();
    row(output, indent, &widths, &header_cells)?;
    rule(output, indent, &widths, '├', '┼', '┤')?;
    for (i, r) in rows.iter().enumerate() {
        row(output, indent, &widths, r)?;
        if i + 1 < rows.len() {
            rule(output, indent, &widths, '├', '┼', '┤')?;
        }
    }
    rule(output, indent, &widths, '└', '┴', '┘')
}

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
