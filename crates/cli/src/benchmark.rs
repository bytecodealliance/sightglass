use anyhow::{Context, Result};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use sightglass_artifact::get_built_engine;
use sightglass_data::{Format, Measurement, Phase};
use sightglass_recorder::measure::Measurements;
use sightglass_recorder::{bench_api::BenchApi, benchmark::benchmark, measure::MeasureType};
use std::{
    fs,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
    process::Command,
    process::Stdio,
};
use structopt::StructOpt;

/// Measure compilation, instantiation, and execution of a Wasm file.
///
/// The total number of samples taken for each Wasm benchmark is `PROCESSES *
/// NUMBER_OF_ITERATIONS_PER_PROCESS`.
#[derive(StructOpt, Debug)]
pub struct BenchmarkCommand {
    /// The benchmark engine(s) with which to run the benchmark.
    ///
    /// This can be either the path to a shared library implementing the
    /// benchmarking engine specification or an engine reference: `[engine
    /// name]@[Git revision]?@[Git repository]?`, e.g. `wasmtime@main`.
    #[structopt(
        long("engine"),
        short("e"),
        value_name = "ENGINE-REF OR PATH",
        empty_values = false,
        default_value = "wasmtime"
    )]
    engines: Vec<String>,

    /// How many processes should we use for each Wasm benchmark?
    #[structopt(long = "processes", default_value = "10", value_name = "PROCESSES")]
    processes: usize,

    /// How many times should we run a benchmark in a single process?
    #[structopt(
        long = "iterations-per-process",
        default_value = "10",
        value_name = "NUMBER_OF_ITERATIONS_PER_PROCESS"
    )]
    iterations_per_process: usize,

    /// Output raw data, rather than the summarized, human-readable analysis
    /// results.
    #[structopt(long)]
    raw: bool,

    /// The format of the raw output data when `--raw` is used. Either 'json' or
    /// 'csv'.
    #[structopt(short = "f", long = "output-format", default_value = "json")]
    output_format: Format,

    /// Path to a file which will contain the output data, or nothing to print
    /// to stdout (default).
    #[structopt(short = "o", long = "output-file")]
    output_file: Option<String>,

    /// The type of measurement to use (wall-cycles, perf-counters, noop) when
    /// recording the benchmark performance.
    #[structopt(long, short, default_value = "wall-cycles")]
    measure: MeasureType,

    /// Pass this flag to only run benchmarks over "small" workloads (rather
    /// than the larger, default workloads).
    ///
    /// Note that not every benchmark program necessarily has a smaller
    /// workload, and this flag may be ignored.
    ///
    /// This should only be used with local "mini" experiments to save time when
    /// prototyping a quick performance optimization, or something similar. The
    /// larger, default workloads should still be considered the ultimate source
    /// of truth, and any cases where results differ between the small and
    /// default workloads, the results from the small workloads should be
    /// ignored.
    #[structopt(long, alias = "small-workload")]
    small_workloads: bool,

    /// The directory to preopen as the benchmark working directory. If the
    /// benchmark accesses files using WASI, it will see this directory as its
    /// current working directory (i.e. `.`). If the working directory is not
    /// specified, the Wasm file's parent directory is used instead.
    #[structopt(short("d"), long("working-dir"), parse(from_os_str))]
    working_dir: Option<PathBuf>,

    /// The path to the Wasm file to compile.
    #[structopt(
        index = 1,
        required = true,
        value_name = "WASMFILE",
        parse(from_os_str)
    )]
    wasm_files: Vec<PathBuf>,

    /// Stop measuring after the given phase (compilation/instantiation/execution).
    #[structopt(long("stop-after"))]
    stop_after_phase: Option<Phase>,

    /// The significance level for confidence intervals. Typical values are 0.01
    /// and 0.05, which correspond to 99% and 95% confidence respectively. This
    /// is ignored when using `--raw` or when there aren't exactly two engines
    /// supplied.
    #[structopt(short, long, default_value = "0.01")]
    significance_level: f64,
}

impl BenchmarkCommand {
    pub fn execute(&self) -> Result<()> {
        anyhow::ensure!(self.processes > 0, "processes must be greater than zero");
        anyhow::ensure!(
            self.iterations_per_process > 0,
            "iterations-per-process must be greater than zero"
        );

        if self.processes == 1 {
            self.execute_in_current_process()
        } else {
            self.execute_in_multiple_processes()
        }
    }

    /// Execute benchmark(s) in the provided engine(s) using the current process.
    pub fn execute_in_current_process(&self) -> Result<()> {
        let mut output_file: Box<dyn Write> = if let Some(file) = self.output_file.as_ref() {
            Box::new(BufWriter::new(fs::File::create(file)?))
        } else {
            Box::new(io::stdout())
        };

        let wasm_files: Vec<_> = self
            .wasm_files
            .iter()
            .map(|f| f.display().to_string())
            .collect();
        let mut all_measurements = vec![];

        for engine in &self.engines {
            let engine_path = get_built_engine(engine)?;
            log::info!("Using benchmark engine: {}", engine_path.display());
            let lib = unsafe { libloading::Library::new(&engine_path)? };
            let mut bench_api = unsafe { BenchApi::new(&lib)? };

            for wasm_file in &wasm_files {
                log::info!("Using Wasm benchmark: {}", wasm_file);

                // Use the provided --working-dir, otherwise find the Wasm file's parent directory.
                let working_dir = self.get_working_directory(&wasm_file)?;
                log::info!("Using working directory: {}", working_dir.display());

                // Read the Wasm bytes.
                let bytes = fs::read(&wasm_file).context("Attempting to read Wasm bytes")?;
                log::debug!("Wasm benchmark size: {} bytes", bytes.len());

                let mut measurements = Measurements::new(this_arch(), engine, wasm_file);
                let mut measure = self.measure.build();

                // Run the benchmark (compilation, instantiation, and execution) several times in
                // this process.
                for i in 0..self.iterations_per_process {
                    let wasm_hash = {
                        use std::collections::hash_map::DefaultHasher;
                        use std::hash::{Hash, Hasher};
                        let mut hasher = DefaultHasher::new();
                        wasm_file.hash(&mut hasher);
                        hasher.finish()
                    };
                    let stdout = format!("stdout-{:x}-{}-{}.log", wasm_hash, std::process::id(), i);
                    let stdout = Path::new(&stdout);
                    let stderr = format!("stderr-{:x}-{}-{}.log", wasm_hash, std::process::id(), i);
                    let stderr = Path::new(&stderr);
                    let stdin = None;

                    benchmark(
                        &mut bench_api,
                        &working_dir,
                        stdout,
                        stderr,
                        stdin,
                        &bytes,
                        self.stop_after_phase.clone(),
                        &mut measure,
                        &mut measurements,
                    )?;

                    self.check_output(Path::new(wasm_file), stdout, stderr)?;
                    measurements.next_iteration();
                }

                all_measurements.extend(measurements.finish());
            }
        }

        self.write_results(&all_measurements, &mut output_file)?;
        Ok(())
    }

    /// Assert that our actual `stdout` and `stderr` match our expectations.
    fn check_output(&self, wasm_file: &Path, stdout: &Path, stderr: &Path) -> Result<()> {
        // If we aren't going through all phases and executing the Wasm, then we
        // won't have any actual output to check.
        if self.stop_after_phase.is_some() {
            return Ok(());
        }

        let wasm_file_dir: PathBuf = if let Some(dir) = wasm_file.parent() {
            dir.into()
        } else {
            ".".into()
        };

        let stdout_expected = wasm_file_dir.join("stdout.expected");
        if stdout_expected.exists() {
            let stdout_expected_data = std::fs::read_to_string(&stdout_expected)
                .with_context(|| format!("failed to read `{}`", stdout_expected.display()))?;
            let stdout_actual_data = std::fs::read_to_string(stdout)
                .with_context(|| format!("failed to read `{}`", stdout.display()))?;
            // Compare lines so that we ignore `\n` on *nix vs `\r\n` on Windows.
            let stdout_expected_data = stdout_expected_data.lines().collect::<Vec<_>>();
            let stdout_actual_data = stdout_actual_data.lines().collect::<Vec<_>>();
            anyhow::ensure!(
                stdout_expected_data == stdout_actual_data,
                "Actual `stdout` does not match the expected `stdout`!\n\
                 * Actual `stdout` is located at `{}`\n\
                 * Expected `stdout` is located at `{}`",
                stdout.display(),
                stdout_expected.display(),
            );
        } else {
            log::warn!(
                "Did not find `{}` for `{}`! Cannot assert that actual \
                 `stdout` matches expectation.",
                stdout_expected.display(),
                wasm_file.display()
            );
        }

        let stderr_expected = wasm_file_dir.join("stderr.expected");
        if stderr_expected.exists() {
            let stderr_expected_data = std::fs::read_to_string(&stderr_expected)
                .with_context(|| format!("failed to read `{}`", stderr_expected.display()))?;
            let stderr_actual_data = std::fs::read_to_string(stderr)
                .with_context(|| format!("failed to read `{}`", stderr.display()))?;
            // Compare lines so that we ignore `\n` on *nix vs `\r\n` on Windows.
            let stderr_expected_data = stderr_expected_data.lines().collect::<Vec<_>>();
            let stderr_actual_data = stderr_actual_data.lines().collect::<Vec<_>>();
            anyhow::ensure!(
                stderr_expected_data == stderr_actual_data,
                "Actual `stderr` does not match the expected `stderr`!\n\
                 * Actual `stderr` is located at `{}`\n\
                 * Expected `stderr` is located at `{}`",
                stderr.display(),
                stderr_expected.display(),
            );
        } else {
            log::warn!(
                "Did not find `{}` for `{}`! Cannot assert that actual \
                 `stderr` matches expectation.",
                stderr_expected.display(),
                wasm_file.display()
            );
        }

        Ok(())
    }

    /// Execute the benchmark(s) by spawning multiple processes. Each of the spawned processes will
    /// run the `execute_in_current_process` function above.
    fn execute_in_multiple_processes(&self) -> Result<()> {
        let mut output_file: Box<dyn Write> = if let Some(file) = self.output_file.as_ref() {
            Box::new(BufWriter::new(fs::File::create(file)?))
        } else {
            Box::new(io::stdout())
        };

        let this_exe =
            std::env::current_exe().context("failed to get the current executable's path")?;

        // Shuffle the order in which we spawn benchmark processes. This helps
        // us avoid some measurement bias from CPU state transitions that aren't
        // constrained within the duration of process execution, like dynamic
        // CPU throttling due to overheating.

        let mut rng = SmallRng::seed_from_u64(0x1337_4242);

        // Worklist that we randomly sample from.
        let mut choices = vec![];

        for engine in &self.engines {
            // Ensure that each of our engines is built before we spawn any
            // child processes (potentially in a different working directory,
            // and therefore potentially invalidating relative paths used here).
            let engine = get_built_engine(engine)?;

            for wasm in &self.wasm_files {
                choices.push((engine.clone(), wasm, self.processes));
            }
        }

        // Accumulated measurements from all of our subprocesses.
        let mut measurements = vec![];

        while !choices.is_empty() {
            let index = rng.gen_range(0, choices.len());
            let (engine, wasm, procs_left) = &mut choices[index];

            let mut command = Command::new(&this_exe);
            command
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::inherit())
                .arg("benchmark")
                .arg("--processes")
                .arg("1")
                .arg("--iterations-per-process")
                .arg(self.iterations_per_process.to_string())
                .arg("--engine")
                .arg(&engine)
                .arg("--measure")
                .arg(self.measure.to_string())
                .arg("--raw")
                .arg("--output-format")
                // Always use JSON when privately communicating with a
                // subprocess.
                .arg(Format::Json.to_string());

            if self.small_workloads {
                command.env("WASM_BENCH_USE_SMALL_WORKLOAD", "1");
            }

            if let Some(phase) = self.stop_after_phase {
                command.arg("--stop-after").arg(phase.to_string());
            }

            command.arg("--").arg(&wasm);

            let output = command
                .output()
                .context("failed to run benchmark subprocess")?;

            anyhow::ensure!(
                output.status.success(),
                "benchmark subprocess did not exit successfully"
            );

            // Parse the subprocess's output and add its measurements to our
            // accumulation.
            measurements.extend(
                serde_json::from_slice::<Vec<Measurement<'_>>>(&output.stdout)
                    .context("failed to read benchmark subprocess's results")?,
            );

            *procs_left -= 1;
            if *procs_left == 0 {
                choices.swap_remove(index);
            }
        }

        self.write_results(&measurements, &mut output_file)?;
        Ok(())
    }

    fn write_results(
        &self,
        measurements: &[Measurement<'_>],
        output_file: &mut dyn Write,
    ) -> Result<()> {
        if self.raw {
            self.output_format.write(measurements, output_file)?;
        } else if self.engines.len() == 2 {
            display_effect_size(measurements, self.significance_level, output_file)?;
        } else {
            display_summaries(measurements, output_file)?;
        }
        Ok(())
    }

    /// Determine the working directory in which to run the benchmark using:
    /// - first, any directory specified with `--working-dir`
    /// - then, the parent directory of the Wasm file
    /// - and if all else fails, the current working directory of the process.
    fn get_working_directory(&self, wasm_file: &impl AsRef<Path>) -> Result<PathBuf> {
        let working_dir = if let Some(dir) = self.working_dir.clone() {
            dir
        } else if let Some(dir) = wasm_file.as_ref().parent() {
            dir.into()
        } else {
            std::env::current_dir().context("failed to get the current working directory")?
        };
        Ok(working_dir)
    }
}

fn this_arch() -> &'static str {
    if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        unimplemented!("please add support for the current target architecture")
    }
}

fn display_effect_size(
    measurements: &[Measurement<'_>],
    significance_level: f64,
    output_file: &mut dyn Write,
) -> Result<()> {
    let mut effect_sizes = sightglass_analysis::effect_size(significance_level, measurements)?;
    let summary = sightglass_analysis::summarize(measurements);

    // Sort the effect sizes so that we focus on statistically
    // significant results before insignificant results and larger
    // relative effect sizes before smaller relative effect sizes.
    effect_sizes.sort_by(|x, y| {
        y.is_significant().cmp(&x.is_significant()).then_with(|| {
            let x_speedup = x.a_speed_up_over_b().0.max(x.b_speed_up_over_a().0);
            let y_speedup = y.a_speed_up_over_b().0.max(y.b_speed_up_over_a().0);
            y_speedup.partial_cmp(&x_speedup).unwrap()
        })
    });

    for effect_size in effect_sizes {
        writeln!(output_file)?;
        writeln!(
            output_file,
            "{} :: {} :: {}",
            effect_size.phase, effect_size.event, effect_size.wasm
        )?;
        writeln!(output_file)?;

        // For readability, trim the shared prefix from our two engine names.
        let end_of_shared_prefix = effect_size
            .a_engine
            .char_indices()
            .zip(effect_size.b_engine.char_indices())
            .find_map(|((i, a), (j, b))| {
                if a == b {
                    None
                } else {
                    debug_assert_eq!(i, j);
                    Some(i)
                }
            })
            .unwrap_or(0);
        let a_engine = &effect_size.a_engine[end_of_shared_prefix..];
        let b_engine = &effect_size.b_engine[end_of_shared_prefix..];

        if effect_size.is_significant() {
            writeln!(
                output_file,
                "  Δ = {:.2} ± {:.2} (confidence = {}%)",
                (effect_size.b_mean - effect_size.a_mean).abs(),
                effect_size.half_width_confidence_interval.abs(),
                (1.0 - significance_level) * 100.0,
            )?;
            writeln!(output_file)?;

            let ratio = effect_size.b_mean / effect_size.a_mean;
            let ratio_ci = effect_size.half_width_confidence_interval / effect_size.a_mean;
            writeln!(
                output_file,
                "  {a_engine} is {ratio_min:.2}x to {ratio_max:.2}x faster than {b_engine}!",
                a_engine = a_engine,
                b_engine = b_engine,
                ratio_min = ratio - ratio_ci,
                ratio_max = ratio + ratio_ci,
            )?;
            let ratio = effect_size.a_mean / effect_size.b_mean;
            let ratio_ci = effect_size.half_width_confidence_interval / effect_size.b_mean;

            writeln!(
                output_file,
                "  {b_engine} is {ratio_min:.2}x to {ratio_max:.2}x faster than {a_engine}!",
                a_engine = a_engine,
                b_engine = b_engine,
                ratio_min = ratio - ratio_ci,
                ratio_max = ratio + ratio_ci,
            )?;
        } else {
            writeln!(output_file, "  No difference in performance.")?;
        }
        writeln!(output_file)?;

        let get_summary = |engine: &str, wasm: &str, phase: Phase, event: &str| {
            summary
                .iter()
                .find(|s| {
                    s.engine == engine && s.wasm == wasm && s.phase == phase && s.event == event
                })
                .unwrap()
        };

        let a_summary = get_summary(
            &effect_size.a_engine,
            &effect_size.wasm,
            effect_size.phase,
            &effect_size.event,
        );
        writeln!(
            output_file,
            "  [{} {:.2} {}] {}",
            a_summary.min, a_summary.mean, a_summary.max, a_engine,
        )?;

        let b_summary = get_summary(
            &effect_size.b_engine,
            &effect_size.wasm,
            effect_size.phase,
            &effect_size.event,
        );
        writeln!(
            output_file,
            "  [{} {:.2} {}] {}",
            b_summary.min, b_summary.mean, b_summary.max, b_engine,
        )?;
    }

    Ok(())
}

fn display_summaries(measurements: &[Measurement<'_>], output_file: &mut dyn Write) -> Result<()> {
    let mut summaries = sightglass_analysis::summarize(measurements);

    summaries.sort_by(|x, y| {
        x.phase
            .cmp(&y.phase)
            .then_with(|| x.wasm.cmp(&y.wasm))
            .then_with(|| x.event.cmp(&y.event))
            .then_with(|| x.engine.cmp(&y.engine))
    });

    let mut last_phase = None;
    let mut last_wasm = None;
    let mut last_event = None;
    for summary in summaries {
        if last_phase != Some(summary.phase) {
            last_phase = Some(summary.phase);
            last_wasm = None;
            last_event = None;
            writeln!(output_file, "{}", summary.phase)?;
        }

        if last_wasm.as_ref() != Some(&summary.wasm) {
            last_wasm = Some(summary.wasm.clone());
            last_event = None;
            writeln!(output_file, "  {}", summary.wasm)?;
        }

        if last_event.as_ref() != Some(&summary.event) {
            last_event = Some(summary.event.clone());
            writeln!(output_file, "    {}", summary.event)?;
        }

        writeln!(
            output_file,
            "      [{} {:.2} {}] {}",
            summary.min, summary.mean, summary.max, summary.engine,
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_summaries() -> Result<()> {
        let fixture = std::fs::read("../../test/fixtures/old-backends.json")
            .context("failed to read fixture file")?;
        let measurements: Vec<Measurement<'_>> = serde_json::from_slice(&fixture)?;
        let mut output = vec![];
        display_summaries(&measurements, &mut output)?;

        let actual = String::from_utf8(output)?;
        eprintln!("=== Actual ===\n{}", actual);

        let expected = r#"
compilation
  benchmarks-next/pulldown-cmark/benchmark.wasm
    cycles
      [696450758 740410589.60 823537015] /tmp/old_backend.so
      [688475571 710846289.20 796284592] /tmp/old_backend_2.so
      [721352134 776890922.40 933479759] /tmp/old_backend_3.so
    nanoseconds
      [239819667 254957035.80 283581244] /tmp/old_backend.so
      [237074550 244777841.50 274198271] /tmp/old_backend_2.so
      [248392822 267517235.10 321437562] /tmp/old_backend_3.so
instantiation
  benchmarks-next/pulldown-cmark/benchmark.wasm
    cycles
      [186145 213469.60 229974] /tmp/old_backend.so
      [200003 220099.00 308810] /tmp/old_backend_2.so
      [203474 233069.30 300269] /tmp/old_backend_3.so
    nanoseconds
      [64098 73506.90 79190] /tmp/old_backend.so
      [68870 75789.90 106337] /tmp/old_backend_2.so
      [70064 80255.30 103395] /tmp/old_backend_3.so
execution
  benchmarks-next/pulldown-cmark/benchmark.wasm
    cycles
      [10334150 12342413.00 14169904] /tmp/old_backend.so
      [10328193 10829803.50 12631959] /tmp/old_backend_2.so
      [10569938 11690281.50 16792916] /tmp/old_backend_3.so
    nanoseconds
      [3558517 4250053.60 4879342] /tmp/old_backend.so
      [3556483 3729210.70 4349778] /tmp/old_backend_2.so
      [3639688 4025470.30 5782529] /tmp/old_backend_3.so
"#;
        eprintln!("=== Expected ===\n{}", expected);

        assert_eq!(actual.trim(), expected.trim());
        Ok(())
    }

    #[test]
    fn test_display_effect_size() -> Result<()> {
        let fixture = std::fs::read("../../test/fixtures/old-vs-new-backend.json")
            .context("failed to read fixture file")?;
        let measurements: Vec<Measurement<'_>> = serde_json::from_slice(&fixture)?;
        let mut output = vec![];
        display_effect_size(&measurements, 0.05, &mut output)?;

        let actual = String::from_utf8(output)?;
        eprintln!("=== Actual ===\n{}", actual);

        let expected = r#"
compilation :: cycles :: benchmarks-next/pulldown-cmark/benchmark.wasm

  Δ = 231879938.88 ± 5920528.32 (confidence = 95%)

  new_backend.so is 0.75x to 0.76x faster than old_backend.so!
  old_backend.so is 1.32x to 1.34x faster than new_backend.so!

  [889384088 935555419.78 1045075629] new_backend.so
  [688072501 703675480.90 826253416] old_backend.so

compilation :: nanoseconds :: benchmarks-next/pulldown-cmark/benchmark.wasm

  Δ = 79845660.57 ± 2038688.33 (confidence = 95%)

  new_backend.so is 0.75x to 0.76x faster than old_backend.so!
  old_backend.so is 1.32x to 1.34x faster than new_backend.so!

  [306252409 322151144.14 359863566] new_backend.so
  [236932712 242305483.57 284514295] old_backend.so

execution :: nanoseconds :: benchmarks-next/pulldown-cmark/benchmark.wasm

  Δ = 467229.61 ± 57708.35 (confidence = 95%)

  new_backend.so is 1.13x to 1.16x faster than old_backend.so!
  old_backend.so is 0.86x to 0.89x faster than new_backend.so!

  [3061587 3240065.98 4419514] new_backend.so
  [3510983 3707295.59 5811112] old_backend.so

execution :: cycles :: benchmarks-next/pulldown-cmark/benchmark.wasm

  Δ = 1356859.60 ± 167590.00 (confidence = 95%)

  new_backend.so is 1.13x to 1.16x faster than old_backend.so!
  old_backend.so is 0.86x to 0.89x faster than new_backend.so!

  [8891120 9409439.69 12834660] new_backend.so
  [10196192 10766299.29 16875960] old_backend.so

instantiation :: cycles :: benchmarks-next/pulldown-cmark/benchmark.wasm

  No difference in performance.

  [191466 207762.01 325810] new_backend.so
  [179617 200451.81 334016] old_backend.so

instantiation :: nanoseconds :: benchmarks-next/pulldown-cmark/benchmark.wasm

  No difference in performance.

  [65929 71540.70 112190] new_backend.so
  [61849 69023.59 115015] old_backend.so
"#;
        eprintln!("=== Expected ===\n{}", expected);

        assert_eq!(actual.trim(), expected.trim());
        Ok(())
    }
}
