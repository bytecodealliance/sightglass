pub mod effect_size;
pub mod keys;
pub mod report_stats;
pub mod summarize;

use sightglass_data::Summary;
use std::io;
use termcolor::{Color, ColorSpec, WriteColor};

/// The column headers for the min/max/mean/median/engine statistics table shared
/// by [`summarize::write`] and [`effect_size::write`].
pub(crate) const STATS_HEADERS: [&str; 5] = ["Min", "Max", "Mean", "Median", "Engine"];

fn fg(color: Color) -> ColorSpec {
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(color));
    spec
}

/// Orange, used for table headers and the effect-size delta.
pub(crate) fn orange() -> ColorSpec {
    fg(Color::Ansi256(208))
}

/// Phases are drawn in light purple.
pub(crate) fn phase_spec() -> ColorSpec {
    fg(Color::Ansi256(141))
}

/// Events are drawn in yellow.
pub(crate) fn event_spec() -> ColorSpec {
    fg(Color::Yellow)
}

/// Benchmarks are drawn in italics.
pub(crate) fn benchmark_spec() -> ColorSpec {
    let mut spec = ColorSpec::new();
    spec.set_italic(true);
    spec
}

/// Engine names/labels are drawn in blue.
pub(crate) fn engine_spec() -> ColorSpec {
    fg(Color::Blue)
}

/// Speed-up ratios are drawn in green.
pub(crate) fn ratio_spec() -> ColorSpec {
    fg(Color::Green)
}

/// The p-value and confidence parenthetical is drawn dimmed and grey.
pub(crate) fn stats_parenthetical_spec() -> ColorSpec {
    let mut spec = fg(Color::Ansi256(245));
    spec.set_dimmed(true);
    spec
}

/// Write `text` to `output` using the given color `spec`, then reset the color.
pub(crate) fn write_in(
    output: &mut dyn WriteColor,
    spec: &ColorSpec,
    text: &str,
) -> io::Result<()> {
    output.set_color(spec)?;
    write!(output, "{text}")?;
    output.reset()
}

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

/// Render the min/max/mean/median/engine statistics `rows` as a colored table.
///
/// Headers are orange, the min and max values are dimmed, and engines are blue.
pub(crate) fn write_stats_table(
    output: &mut dyn WriteColor,
    indent: &str,
    rows: &[Vec<String>],
) -> io::Result<()> {
    let dimmed = {
        let mut spec = ColorSpec::new();
        spec.set_dimmed(true);
        spec
    };
    let plain = ColorSpec::new();
    // One spec per column: Min, Max (dimmed), Mean, Median (plain), Engine (blue).
    let column_specs = [dimmed.clone(), dimmed, plain.clone(), plain, engine_spec()];
    write_table(
        output,
        indent,
        &STATS_HEADERS,
        &orange(), // orange headers
        &column_specs,
        rows,
    )
}

/// Render `rows` as a table drawn with Unicode box-drawing characters, prefixing
/// every line with `indent`.
///
/// Each column is as wide as its widest cell (including the `headers`), cells are
/// left-aligned, and every row -- header and data alike -- is separated by a
/// horizontal rule. Header cells are written with `header_spec` and each data
/// cell with its column's spec from `column_specs`:
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
    output: &mut dyn WriteColor,
    indent: &str,
    headers: &[&str],
    header_spec: &ColorSpec,
    column_specs: &[ColorSpec],
    rows: &[Vec<String>],
) -> io::Result<()> {
    let mut widths: Vec<usize> = headers.iter().map(|h| h.chars().count()).collect();
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            widths[i] = widths[i].max(cell.chars().count());
        }
    }

    fn rule(
        output: &mut dyn WriteColor,
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
        output: &mut dyn WriteColor,
        indent: &str,
        widths: &[usize],
        cells: &[String],
        specs: &[ColorSpec],
    ) -> io::Result<()> {
        write!(output, "{indent}│")?;
        for ((cell, width), spec) in cells.iter().zip(widths).zip(specs) {
            write!(output, " ")?;
            output.set_color(spec)?;
            write!(output, "{cell:<width$}")?;
            output.reset()?;
            write!(output, " │")?;
        }
        writeln!(output)
    }

    let header_cells: Vec<String> = headers.iter().map(|h| h.to_string()).collect();
    let header_specs: Vec<ColorSpec> = vec![header_spec.clone(); headers.len()];

    rule(output, indent, &widths, '┌', '┬', '┐')?;
    row(output, indent, &widths, &header_cells, &header_specs)?;
    rule(output, indent, &widths, '├', '┼', '┤')?;
    for (i, r) in rows.iter().enumerate() {
        row(output, indent, &widths, r, column_specs)?;
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
