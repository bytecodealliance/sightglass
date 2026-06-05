// Run `$BENCHMARKS` under Valgrind's Callgrind with the `callgrind` measure
// for the `callgrind` workflow; see `callgrind.yml`.

import { strict as assert } from "node:assert";
import { spawnSync } from "node:child_process";
import * as fs from "node:fs";

/// Run a command with inherited stdio, throwing if it fails.
function exec(file, args) {
  const result = spawnSync(file, args, { stdio: "inherit" });
  assert.equal(
    result.status,
    0,
    `\`${file} ${args.join(" ")}\` exited with ${result.status}`,
  );
}

/// Shorten a benchmark's wasm path to a human-friendly name, e.g.
/// `benchmarks/bz2/benchmark.wasm` to `bz2` and
/// `benchmarks/cm-online-stats/cm-online-stats.wasm` to `cm-online-stats`.
function shortName(wasm) {
  const name = wasm
    .replace(/^.*benchmarks\//, "")
    .replace(/\.wasm$/, "")
    .replace(/\/benchmark$/, "");
  const parts = name.split("/");
  return parts.length === 2 && parts[0] === parts[1] ? parts[0] : name;
}

/// Run the benchmarks, render a summary table, check the results, and (with
/// at least three iterations) generate an HTML report. Everything is left in
/// the `callgrind-results` directory for upload as a workflow artifact.
async function main() {
  const benchmarks = (process.env.BENCHMARKS || process.env.DEFAULT_BENCHMARKS)
    .split(/\s+/)
    .filter(Boolean);
  const iterations = parseInt(process.env.ITERATIONS || "1", 10);

  // `sightglass-cli` launches each benchmark process under Valgrind's
  // Callgrind itself when the `callgrind` measure is selected. (Valgrind is
  // installed by the workflow before sightglass is even built: the Valgrind
  // headers must be present at build time.)
  const run = spawnSync(
    "target/debug/sightglass-cli",
    [
      "benchmark",
      "--engine",
      "engines/wasmtime/libengine.so",
      "--measure",
      "callgrind",
      "--processes",
      "1",
      "--iterations-per-process",
      String(iterations),
      "--small-workloads",
      "--raw",
      "--output-format",
      "json",
      "--",
      ...benchmarks,
    ],
    {
      stdio: ["ignore", "pipe", "inherit"],
      encoding: "utf8",
      maxBuffer: 64 * 1024 * 1024,
    },
  );
  console.log(run.stdout);
  assert.equal(run.status, 0, `sightglass-cli exited with ${run.status}`);

  fs.mkdirSync("callgrind-results");
  fs.writeFileSync("callgrind-results/results.json", run.stdout);
  for (const file of fs.readdirSync(".")) {
    if (file.startsWith("callgrind.out.")) {
      fs.renameSync(file, `callgrind-results/${file}`);
    }
  }

  // Group the measurements by benchmark.
  const byWasm = new Map();
  for (const m of JSON.parse(run.stdout)) {
    if (m.event !== "callgrind-ir") {
      continue;
    }
    if (!byWasm.has(m.wasm)) {
      byWasm.set(m.wasm, []);
    }
    byWasm.get(m.wasm).push(m);
  }

  // Display each benchmark by its short name, unless two benchmarks share one.
  const displayNames = new Map();
  {
    const occurrences = new Map();
    for (const wasm of byWasm.keys()) {
      const name = shortName(wasm);
      occurrences.set(name, (occurrences.get(name) ?? 0) + 1);
    }
    for (const wasm of byWasm.keys()) {
      const name = shortName(wasm);
      displayNames.set(wasm, occurrences.get(name) === 1 ? name : wasm);
    }
  }

  // A phase's instruction counts across iterations; deterministic counts
  // collapse to a single value.
  const phaseCounts = (ms, phase) => {
    const counts = new Set(
      ms.filter((m) => m.phase === phase).map((m) => m.count),
    );
    return [...counts].join(" / ");
  };

  // Render a table of the results on the workflow run's summary page. This
  // happens before the checks below so that a failing run still reports what
  // it measured.
  const summary = [
    "### Callgrind instruction counts (`callgrind-ir`)",
    "",
    `Engine: \`wasmtime@${process.env.WASMTIME_SHA}\``,
    "",
    "| Benchmark | Compilation | Instantiation | Execution |",
    "| --- | ---: | ---: | ---: |",
    ...[...byWasm.entries()].map(([wasm, ms]) => {
      const phases = ["Compilation", "Instantiation", "Execution"]
        .map((phase) => phaseCounts(ms, phase))
        .join(" | ");
      return `| ${displayNames.get(wasm)} | ${phases} |`;
    }),
    "",
    "Distinct counts across iterations are shown separated by ` / `; the raw" +
      " per-phase Callgrind dumps and the JSON measurements are available in" +
      " the `callgrind-results` artifact.",
    "",
  ].join("\n");
  fs.appendFileSync(process.env.GITHUB_STEP_SUMMARY, summary);

  // Every benchmark must have recorded an instruction count for each of its
  // three phases on every iteration.
  assert.ok(byWasm.size >= 1, "no benchmarks were measured");
  for (const [wasm, ms] of byWasm) {
    assert.equal(
      ms.length,
      3 * iterations,
      `wrong number of instruction counts for ${wasm}`,
    );
  }

  // With enough samples, also generate an HTML report.
  if (iterations >= 3) {
    exec("target/debug/sightglass-cli", [
      "report",
      "-i",
      "json",
      "--event",
      "callgrind-ir",
      "--phase",
      "execution",
      "-o",
      "callgrind-results/report.html",
      "callgrind-results/results.json",
    ]);
  }
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
