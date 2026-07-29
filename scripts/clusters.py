#!/usr/bin/env python3
"""Render a per-cluster stacked bar chart for a Sightglass benchmark clustering.

Given the three CSVs produced elsewhere in the PCA workflow, this draws one
stacked bar chart per cluster (`./cluster-<N>.svg`) comparing, for every
benchmark in the cluster:

  * the number of wasm instructions dynamically executed (bottom segment), and
  * the average number of native instructions retired while compiling the
    benchmark (top segment).

### Inputs

1. `metrics.csv`: Output of `cargo run -- pca-metrics`.

2. `compilation.csv`: Output of `cargo run -- benchmark --benchmark-phase
   compilation --raw -f csv`.

3. `clusters.csv`: Result of `scripts/pca.R`.

### Example

    cargo run -- pca-metrics -o metrics.csv -- benchmarks/all.suite

    cargo run -- benchmark --benchmark-phase compilation \
        --raw -f csv -o compilation.csv -- benchmarks/all.suite

    ./scripts/pca.R metrics.csv          # writes clusters.csv

    ./scripts/clusters.py metrics.csv compilation.csv clusters.csv

Requires `wasm-tools` and `Rscript` to be installed, as well as R's `ggplot2`
and `ggpattern` libaries.
"""

import argparse
import csv
import os
import shutil
import subprocess
import sys
import tempfile

PROG = os.path.basename(sys.argv[0]) or "clusters.py"

# The metrics column holding each benchmark's dynamic wasm instruction count.
WASM_EXEC_COLUMN = "dynamic_total_inst_count"

# The event name (in compilation.csv) whose count is the native instructions
# retired during compilation.
COMPILATION_EVENT = "instructions-retired"

# The repository root, used to resolve the (repo-relative) benchmark paths in
# clusters.csv when clusters.py is run from somewhere other than the root.
REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def warn(message):
    """Print a warning to stderr without aborting."""
    print(f"{PROG}: warning: {message}", file=sys.stderr)


def die(message, code=2):
    """Print an error to stderr and exit."""
    print(f"{PROG}: error: {message}", file=sys.stderr)
    sys.exit(code)


def benchmark_label(path):
    """Turn a benchmark path into a short label.

    Strip the shared "benchmarks/" prefix and the ".wasm" suffix and keep the
    file's stem, falling back to the parent directory for the generic
    "benchmark.wasm" name.
    """
    stripped = path[len("benchmarks/"):] if path.startswith("benchmarks/") else path
    parts = stripped.split("/")
    stem = parts[-1]
    if stem.endswith(".wasm"):
        stem = stem[: -len(".wasm")]
    if stem == "benchmark" and len(parts) >= 2:
        return parts[-2]
    return stem


def read_csv_dicts(path, required_columns, what):
    """Read a CSV into a list of row dicts, validating required columns exist.

    `what` names the file in error messages. Exits with a helpful message if the
    file is missing/unreadable or is missing any required column.
    """
    if not os.path.isfile(path):
        die(f"{what} file not found: {path}")
    try:
        with open(path, newline="") as f:
            reader = csv.DictReader(f)
            fieldnames = reader.fieldnames or []
            missing = [c for c in required_columns if c not in fieldnames]
            if missing:
                die(
                    f"{what} file {path} is missing required column(s) "
                    f"{', '.join(missing)}; found columns: "
                    f"{', '.join(fieldnames) or '(none)'}"
                )
            return list(reader)
    except OSError as e:
        die(f"could not read {what} file {path}: {e}")


def read_clusters(path):
    """Read clusters.csv into an ordered list of (wasm_path, cluster_int).

    Preserves file order. Exits on a non-integer cluster value.
    """
    rows = read_csv_dicts(path, ["wasm", "cluster"], "clusters")
    out = []
    for i, row in enumerate(rows, start=2):  # start=2: line 1 is the header
        wasm = (row["wasm"] or "").strip()
        raw = (row["cluster"] or "").strip()
        if not wasm:
            warn(f"clusters.csv line {i}: empty wasm path; skipping")
            continue
        try:
            cluster = int(raw)
        except ValueError:
            die(
                f"clusters.csv line {i}: cluster value {raw!r} for {wasm} "
                f"is not an integer"
            )
        out.append((wasm, cluster))
    if not out:
        die("clusters.csv contained no usable rows")
    return out


def read_metrics(path):
    """Read metrics.csv into {wasm_path: wasm_instructions_executed}."""
    rows = read_csv_dicts(path, ["benchmark", WASM_EXEC_COLUMN], "metrics")
    out = {}
    for i, row in enumerate(rows, start=2):
        wasm = (row["benchmark"] or "").strip()
        raw = (row[WASM_EXEC_COLUMN] or "").strip()
        if not wasm:
            continue
        try:
            out[wasm] = float(raw)
        except ValueError:
            warn(
                f"metrics.csv line {i}: {WASM_EXEC_COLUMN} value {raw!r} for "
                f"{wasm} is not a number; ignoring this benchmark"
            )
    return out


def read_compilation(path):
    """Read compilation.csv into `{wasm_path: mean_native_instructions_retired}`.

    Averages the `count` of every `instructions-retired` row of the compilation
    phase for each benchmark (i.e. across all processes/iterations).
    """
    rows = read_csv_dicts(
        path, ["wasm", "phase", "event", "count"], "compilation"
    )
    sums = {}
    counts = {}
    for i, row in enumerate(rows, start=2):
        if (row["event"] or "").strip() != COMPILATION_EVENT:
            continue
        if (row["phase"] or "").strip().lower() != "compilation":
            continue
        wasm = (row["wasm"] or "").strip()
        raw = (row["count"] or "").strip()
        try:
            value = float(raw)
        except ValueError:
            warn(
                f"compilation.csv line {i}: count {raw!r} for {wasm} is not a "
                f"number; ignoring this sample"
            )
            continue
        sums[wasm] = sums.get(wasm, 0.0) + value
        counts[wasm] = counts.get(wasm, 0) + 1
    return {wasm: sums[wasm] / counts[wasm] for wasm in sums}


def resolve_wasm(wasm_path):
    """Locate a benchmark's `.wasm` on disk, or return `None`.

    `clusters.csv` holds repo-relative paths (e.g. `benchmarks/foo/benchmark.wasm`),
    so try the path as given (relative to the cwd) and then relative to the
    repository root.
    """
    candidates = [wasm_path, os.path.join(REPO_ROOT, wasm_path)]
    for candidate in candidates:
        if os.path.isfile(candidate):
            return candidate
    return None


def has_file_reading_wasi_import(skeleton):
    """True if the wasm skeleton imports a WASI file-reading interface.

    Covers both the core-module (WASI preview1) and component (WASI preview2)
    forms:

      * preview1: an import of `wasi_snapshot_preview1` `path_open` or `fd_read`
        (opening/reading a file descriptor), and
      * preview2: an import of the `wasi:filesystem` interface (`types` or
        `preopens`), which is what a component uses to open and read files.
    """
    for line in skeleton.splitlines():
        if "import" not in line:
            continue
        if '"wasi_snapshot_preview1"' in line and (
            '"path_open"' in line or '"fd_read"' in line
        ):
            return True
        if '"wasi:filesystem/types' in line or '"wasi:filesystem/preopens' in line:
            return True
    return False


def has_input_file(wasm_on_disk):
    """True if an input workload file sits next to the .wasm.

    Matches a sibling named `<stem>.input` or `default.input`, optionally with a
    trailing extension (e.g. `default.input.md`, which `pulldown-cmark` reads).
    """
    directory = os.path.dirname(wasm_on_disk) or "."
    stem = os.path.basename(wasm_on_disk)
    if stem.endswith(".wasm"):
        stem = stem[: -len(".wasm")]
    bases = (stem + ".input", "default.input")
    try:
        entries = os.listdir(directory)
    except OSError:
        return False
    for name in entries:
        for base in bases:
            if name == base or name.startswith(base + "."):
                return True
    return False


def reads_input_via_wasi(wasm_path):
    """Determine whether a benchmark reads its input via WASI file I/O.

    This is the conjunction of two signals:

      1. the wasm imports a WASI file-reading interface (found by inspecting
         `wasm-tools wasm2wat --skeleton` output), and
      2. an input workload file (`<stem>.input` or `default.input`, optionally
         with a trailing extension such as `default.input.md`) is present next
         to the benchmark.

    A benchmark that imports file I/O but ships no input file (it embeds or
    generates its workload), or that has an input file but no file-reading
    imports, is treated as *not* reading its input via WASI file I/O. If the
    determination cannot be made (missing `.wasm`, or wasm-tools failure) a
    warning is issued and the benchmark is treated as not reading via WASI.
    """
    on_disk = resolve_wasm(wasm_path)
    if on_disk is None:
        warn(
            f"cannot locate {wasm_path} on disk; assuming it does not read "
            f"input via WASI file I/O"
        )
        return False

    try:
        result = subprocess.run(
            ["wasm-tools", "wasm2wat", "--skeleton", on_disk],
            capture_output=True,
            text=True,
            timeout=120,
        )
    except (OSError, subprocess.SubprocessError) as e:
        warn(f"wasm-tools failed on {wasm_path}: {e}; assuming no WASI file I/O")
        return False
    if result.returncode != 0:
        warn(
            f"wasm-tools exited {result.returncode} on {wasm_path}: "
            f"{result.stderr.strip()}; assuming no WASI file I/O"
        )
        return False

    return has_file_reading_wasi_import(result.stdout) and has_input_file(on_disk)


def _r_string_vector(values):
    escaped = ['"' + v.replace("\\", "\\\\").replace('"', '\\"') + '"' for v in values]
    return "c(" + ", ".join(escaped) + ")"


def _r_numeric_vector(values):
    return "c(" + ", ".join(repr(float(v)) for v in values) + ")"


def _r_logical_vector(values):
    return "c(" + ", ".join("TRUE" if v else "FALSE" for v in values) + ")"


# The R script to render one cluster.
#
# It expects `labels`, `wasm_insts`, `native_insts`, `reads_wasi`, `cluster_id`,
# and `out_file` to be defined above it.
#
# Each bar stacks compilation beneath execution, drawn as explicit
# `geom_rect_pattern`s with `y` extents in linear space so the stack stays
# correct while only *displayed* on a log axis (`position="stack"` would stack
# in log space, which is meaningless).
#
# Segments are differentiated by fill and pattern type.
#
# WASI file I/O shows as a bold black border and saturated fill for benchmarks
# that read input via WASI, and a thin grey border and faded fill otherwise.
_R_BODY = r"""
suppressMessages({
  library(ggplot2)
  library(ggpattern)
  library(svglite)
})

n <- length(labels)
totals <- wasm_insts + native_insts

# Sort the x axis from least to most total (wasm executed + native compiled).
o <- order(totals)
labels <- labels[o]; wasm_insts <- wasm_insts[o]; native_insts <- native_insts[o]
reads_wasi <- reads_wasi[o]; totals <- totals[o]

# Baseline for the log axis: a power of ten below the smallest bottom
# (compilation) value, so the shortest bar still spans about a decade.
floor_val <- 10 ^ (floor(log10(min(native_insts))) - 1)
ymax <- max(totals) * 1.3

seg_levels  <- c("Execution (Wasm instructions)",
                 "Compilation (native instructions)")
wasi_levels <- c("WASI file I/O input", "Hard-coded input")

# Long format with explicit y extents. Compilation is the bottom segment
# (floor..native); Execution is stacked on top (native..native+wasm). Computing
# the stack here (linear space) keeps it correct on the log-*displayed* axis.
idx <- seq_len(n)
df <- data.frame(
  xnum = rep(idx, 2),
  segment = factor(rep(seg_levels, each = n), levels = seg_levels),
  wasi = factor(rep(ifelse(reads_wasi, wasi_levels[1], wasi_levels[2]), 2),
                levels = wasi_levels),
  ymin = c(native_insts, rep(floor_val, n)),
  ymax = c(native_insts + wasm_insts, native_insts)
)
df$xmin <- df$xnum - 0.4
df$xmax <- df$xnum + 0.4
# Drop any zero-height segment so it doesn't draw a hairline.
df <- df[df$ymax > df$ymin, , drop = FALSE]

fills    <- setNames(c("#3b6fb6", "#e08214"), seg_levels)   # background per segment
patterns <- setNames(c("stripe", "circle"), seg_levels)     # execution = stripes, compilation = dots
borders  <- setNames(c("black", "grey60"), wasi_levels)     # bar outline colour (WASI only)
lwds     <- setNames(c(1.4, 0.3), wasi_levels)              # bar outline width (WASI only)
alphas   <- setNames(c(1, 0.4), wasi_levels)                # desaturate the hard-coded fills

# Legend keys render borders thinner than the big bars, so use heavier weights in
# the legend override so the WASI key visually matches the bars' boldness.
legend_lwds <- setNames(c(2.75, 0.5), wasi_levels)

# The pattern is drawn in one constant colour, independent of WASI, so the
# pattern's weight never varies between benchmarks; only the bar outline does.
pat_col <- "grey20"
fmt <- scales::label_number(scale_cut = scales::cut_short_scale())

# A cluster may contain only one WASI category, in which case the "Input source"
# legend draws a single key. Subset the per-key legend overrides to the
# categories actually present so their length matches the number of keys.
wasi_present <- wasi_levels[wasi_levels %in% as.character(df$wasi)]

p <- ggplot(df) +
  geom_rect_pattern(
    aes(xmin = xmin, xmax = xmax, ymin = ymin, ymax = ymax,
        fill = segment, pattern = segment, alpha = wasi,
        colour = wasi, linewidth = wasi),
    pattern_fill = pat_col, pattern_colour = pat_col, pattern_alpha = 1,
    pattern_density = 0.2, pattern_spacing = 0.02,
    pattern_key_scale_factor = 0.6
  ) +
  scale_fill_manual(values = fills, name = "Stacked bar segment") +
  scale_pattern_manual(values = patterns, name = "Stacked bar segment") +
  scale_alpha_manual(values = alphas, name = "Input source") +
  scale_colour_manual(values = borders, name = "Input source") +
  scale_linewidth_manual(values = lwds, name = "Input source") +
  scale_x_continuous(breaks = idx, labels = labels,
                     expand = expansion(add = 0.6)) +
  scale_y_log10(name = "Instructions (log scale)", labels = fmt) +
  coord_cartesian(ylim = c(floor_val, ymax)) +
  labs(title = sprintf("Cluster %d", cluster_id)) +
  guides(
    fill = guide_legend(order = 1, override.aes = list(
      alpha = 1, colour = "grey30", linewidth = 0.4)),
    pattern = guide_legend(order = 1),
    colour = guide_legend(order = 2, override.aes = list(
      fill = "grey55", pattern = "none",
      alpha = alphas[wasi_present], linewidth = legend_lwds[wasi_present])),
    alpha = "none", linewidth = "none"
  ) +
  theme_minimal(base_size = 12) +
  theme(
    legend.position = "right",
    axis.text.x = element_text(angle = 45, hjust = 1),
    panel.grid.major.x = element_blank(),
    panel.grid.minor.x = element_blank(),
    panel.grid.minor.y = element_blank(),
    plot.title = element_text(face = "bold", hjust = 0.5)
  )

# Width scales with the number of bars; the extra ~4 inches leaves room for the
# two legends on the right.
width <- max(7, 0.45 * n + 4)
ggsave(out_file, plot = p, width = width, height = 6.5,
       device = svglite::svglite)
"""


def render_cluster(cluster_id, labels, wasm_insts, native_insts, reads_wasi):
    """Fork out to `Rscript` to render `./cluster-<cluster_id>.svg`.

    Raises `RuntimeError` if `Rscript` fails.
    """
    out_file = f"cluster-{cluster_id}.svg"
    header = (
        f"labels <- {_r_string_vector(labels)}\n"
        f"wasm_insts <- {_r_numeric_vector(wasm_insts)}\n"
        f"native_insts <- {_r_numeric_vector(native_insts)}\n"
        f"reads_wasi <- {_r_logical_vector(reads_wasi)}\n"
        f"cluster_id <- {int(cluster_id)}L\n"
        f'out_file <- "{out_file}"\n'
    )
    script = header + _R_BODY

    tmp = tempfile.NamedTemporaryFile(
        mode="w", suffix=".R", prefix=f"cluster-{cluster_id}-", delete=False
    )
    try:
        tmp.write(script)
        tmp.close()
        result = subprocess.run(
            ["Rscript", "--vanilla", tmp.name],
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            # Keep the script around for debugging when rendering fails.
            raise RuntimeError(
                f"Rscript exited {result.returncode} (script kept at "
                f"{tmp.name}):\n{result.stderr.strip()}"
            )
    except Exception:
        raise
    else:
        os.unlink(tmp.name)
    return out_file


def build_cluster_rows(cluster_id, wasm_paths, metrics, compilation, wasi):
    """Assemble the plotting rows for one cluster.

    Benchmarks missing from `metrics.csv` or `compilation.csv` are dropped with
    a warning (rather than aborting), so the rest of the cluster still renders.
    Returns four parallel lists: `labels`, `wasm_insts`, `native_insts`,
    `reads_wasi`.
    """
    labels, wasm_insts, native_insts, reads_wasi = [], [], [], []
    for wasm in wasm_paths:
        if wasm not in metrics:
            warn(
                f"cluster {cluster_id}: {wasm} is missing from metrics.csv "
                f"(no {WASM_EXEC_COLUMN}); skipping this benchmark"
            )
            continue
        if wasm not in compilation:
            warn(
                f"cluster {cluster_id}: {wasm} is missing from compilation.csv "
                f"(no {COMPILATION_EVENT} rows); skipping this benchmark"
            )
            continue
        labels.append(benchmark_label(wasm))
        wasm_insts.append(metrics[wasm])
        native_insts.append(compilation[wasm])
        reads_wasi.append(wasi.get(wasm, False))
    return labels, wasm_insts, native_insts, reads_wasi


def parse_args(argv):
    parser = argparse.ArgumentParser(
        prog=PROG,
        description=(
            "Render one stacked bar chart per benchmark cluster "
            "(./cluster-<N>.svg): wasm instructions executed and "
            "native instructions retired during compilation."
        ),
        epilog=(
            "example:\n\n"
            "  cargo run -- pca-metrics -o metrics.csv -- benchmarks/all.suite\n"
            "  cargo run -- benchmark --benchmark-phase compilation "
            "--raw -f csv -o compilation.csv -- benchmarks/all.suite\n"
            "  ./scripts/pca.R metrics.csv          # writes clusters.csv\n"
            "  ./scripts/clusters.py metrics.csv compilation.csv clusters.csv"
        ),
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "metrics_csv",
        help="pca-metrics CSV",
    )
    parser.add_argument(
        "compilation_csv",
        help="compilation benchmark CSV",
    )
    parser.add_argument(
        "clusters_csv",
        help="clusters CSV",
    )
    return parser.parse_args(argv)


def main(argv=None):
    args = parse_args(argv)

    # These external tools are required for the whole run, so check them up
    # front with an actionable message rather than failing benchmark-by-benchmark.
    if shutil.which("wasm-tools") is None:
        die(
            "wasm-tools not found on PATH; it is required to detect WASI "
            "file I/O (install from https://github.com/bytecodealliance/wasm-tools)"
        )
    if shutil.which("Rscript") is None:
        die("Rscript not found on PATH; it is required to render the charts")

    clusters = read_clusters(args.clusters_csv)
    metrics = read_metrics(args.metrics_csv)
    compilation = read_compilation(args.compilation_csv)
    if not compilation:
        warn(
            f"compilation.csv had no '{COMPILATION_EVENT}' rows for the "
            f"compilation phase; every cluster will be skipped"
        )

    # Determine WASI file I/O once per unique benchmark (hard-coded into the R
    # invocation for each cluster it appears in).
    unique_wasm = list(dict.fromkeys(wasm for wasm, _ in clusters))
    wasi = {wasm: reads_input_via_wasi(wasm) for wasm in unique_wasm}

    # Group benchmarks by cluster, preserving first-seen order of clusters.
    grouped = {}
    for wasm, cluster in clusters:
        grouped.setdefault(cluster, []).append(wasm)

    rendered, skipped = 0, 0
    for cluster_id in sorted(grouped):
        # Any error inside a cluster becomes a warning so the remaining clusters
        # still get processed.
        try:
            labels, wasm_insts, native_insts, reads_wasi = build_cluster_rows(
                cluster_id, grouped[cluster_id], metrics, compilation, wasi
            )
            if not labels:
                warn(
                    f"cluster {cluster_id}: no benchmarks with both metrics and "
                    f"compilation data; skipping"
                )
                skipped += 1
                continue
            out_file = render_cluster(
                cluster_id, labels, wasm_insts, native_insts, reads_wasi
            )
            n_wasi = sum(reads_wasi)
            print(
                f"wrote {out_file} ({len(labels)} benchmark(s), "
                f"{n_wasi} reading input via WASI file I/O)"
            )
            rendered += 1
        except Exception as e:  # noqa: BLE001 - deliberately keep going
            warn(f"cluster {cluster_id}: {e}")
            skipped += 1

    print(f"\n{rendered} cluster chart(s) written, {skipped} skipped.")
    return 0 if skipped == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
