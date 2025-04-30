#!/usr/bin/env python3
#
# /// script
# dependencies = [
#     "pandas",
#     "altair",
#     "jinja2"
# ]
# ///

import argparse
import os
import pandas as pd
import altair as alt
from dataclasses import dataclass
import re
import sys
import pathlib

# our usage isn't abusing this, suppress noisy warnings
pd.options.mode.chained_assignment = None


@dataclass
class RelativeStats:
    cv: float
    p25: float
    p25_delta_pct: float


@dataclass
class Stats:
    cv: float
    p25: float
    relative: dict[str, RelativeStats]


def wasm_path_to_benchmark_name(wasm_path: str) -> str:
    splits = wasm_path.split("/")
    if splits[-1] == "benchmark.wasm":
        # E.g. noop/benchmark.wasm -> noop
        return splits[-2]
    else:
        # E.g. libsodium/libsodium-box7.wasm -> libsodium-box7
        return splits[-1].replace(".wasm", "")


def parse_single_input(path, prefix, pass_num=1, measure="cycles"):
    df = pd.read_json(path)
    df = df[df["event"] == measure]
    df["pass"] = pass_num
    df["prefix"] = prefix
    return df


def parse_inputs(inputs, measure="cycles"):
    """Yield a chart for each pass for this prefix"""
    if len(inputs) == 1 and not inputs[0].endswith(".json"):
        # assume this is a directory; try to use all JSON files
        import glob

        file_inputs = list(sorted(glob.glob(f"{inputs[0]}/*.json")))
        return parse_inputs(file_inputs, measure)

    # list of files now; organize by prefix if detected
    RE_NUMBERED = re.compile(r"(?P<prefix>.+)-(?P<number>\d+).json")
    df = pd.DataFrame()
    for path in inputs:
        path = pathlib.Path(path)
        if not path.exists():
            print(f"{path} not found!")
            return sys.exit(1)

        match = RE_NUMBERED.match(path.name)
        if match:
            prefix = match["prefix"]
            pass_num = int(match["number"])
            df = pd.concat([df, parse_single_input(path, prefix, pass_num)])
        else:
            prefix = path.stem
            df = pd.concat([df, parse_single_input(path, prefix)])

    return df


def compute_stats(df, benchmark, baseline):
    # select only rows for this benchmark
    df = df[df["benchmark"] == benchmark]

    baseline_df = df[df["prefix"] == baseline]
    prefixes = df[df["prefix"] != baseline]["prefix"].unique()

    baseline_p25 = baseline_df["count"].quantile(0.25)
    baseline_mean = baseline_df["count"].mean()
    baseline_cv = (baseline_df["count"].std() / baseline_mean) * 100
    stats = Stats(cv=baseline_cv, p25=baseline_p25, relative={})
    for prefix in prefixes:
        prefix_df = df[df["prefix"] == prefix]
        prefix_p25 = prefix_df["count"].quantile(0.25)
        prefix_mean = prefix_df["count"].mean()
        prefix_cv = (prefix_df["count"].std() / prefix_mean) * 100
        p25_delta_pct = (
            (prefix_p25 - baseline_p25) / ((baseline_p25 + prefix_p25) / 2)
        ) * 100
        rel_stats = RelativeStats(
            cv=prefix_cv, p25=prefix_p25, p25_delta_pct=p25_delta_pct
        )
        stats.relative[prefix] = rel_stats

    return stats


def plot_benchmark(df, baseline, benchmark):
    # select only rows for this benchmark
    df = df[df["benchmark"] == benchmark]

    chart1 = (
        alt.Chart(df)
        .mark_boxplot()
        .encode(
            y=alt.Y("prefix", title=None),
            x=alt.X("count:Q", title="Count (Cycles)"),
            color=alt.Color("prefix"),
        )
    )

    df_baseline = df[df["prefix"] == baseline]
    baseline_p25 = df_baseline["count"].quantile(0.25)
    df["pct_diff_from_p25"] = (df["count"] - baseline_p25) / baseline_p25 * 100

    chart2 = (
        alt.Chart(df)
        .mark_boxplot()
        .encode(
            y=alt.Y("prefix", title=None),
            x=alt.X("pct_diff_from_p25", title="Percent Difference from Baseline p25"),
            color=alt.Color("prefix"),
        )
    )

    return alt.hconcat(chart1, chart2).properties(title=f"{benchmark}")


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("inputs", nargs="+", help="Directory or JSON files to analyze")
    parser.add_argument(
        "-b",
        "--baseline",
        default=None,
        help="prefix of baseline to use (by default look for one containing 'baseline')",
    )
    parser.add_argument(
        "-o", "--output", default="viz.html", help="HTML File to output"
    )
    args = parser.parse_args()
    return args


def render(args, tmpl_data):
    from jinja2 import Environment, FileSystemLoader

    env = Environment(
        loader=FileSystemLoader(os.path.abspath(os.path.dirname(__file__)))
    )
    viz_template = env.get_template("viz.jinja")
    rendered = viz_template.render(**tmpl_data)
    with open(args.output, "w") as out:
        out.write(rendered)
    print(f"Wrote {args.output}")


def main():
    args = parse_args()
    df = parse_inputs(args.inputs)

    # add column for benchmark
    df["benchmark"] = df.apply(
        lambda row: wasm_path_to_benchmark_name(row["wasm"]), axis=1
    )

    tmpl_data = {}
    benchmarks = df["benchmark"].unique()
    baseline = (
        next((p for p in df["prefix"].unique() if "baseline" in p), None)
        or benchmarks[0]
    )

    # reorg prefixes so that baseline is first
    prefixes = df["prefix"].unique()
    for i, prefix in enumerate(prefixes):
        if prefix == baseline:
            prefixes[i] = prefixes[0]
            prefixes[0] = baseline
            break
    tmpl_data["prefixes"] = prefixes
    tmpl_data["baseline"] = baseline
    tmpl_data["benchmarks"] = []
    for benchmark in benchmarks:
        chart = plot_benchmark(df, baseline, benchmark)
        tmpl_data["benchmarks"].append(
            {
                "name": benchmark,
                "baseline": baseline,
                "stats": compute_stats(df, benchmark, baseline),
                "chart": {
                    "id": benchmark,
                    "json": chart.to_json(),
                },
            }
        )
    render(args, tmpl_data)


if __name__ == "__main__":
    main()
