use crate::suite::BenchmarkOrSuite;
use anyhow::{anyhow, Context, Result};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use sightglass_data::{Format, Measurement, Phase};
use sightglass_recorder::bench_api::Engine;
use sightglass_recorder::cpu_affinity::bind_to_single_core;
use sightglass_recorder::measure::multi::MultiMeasure;
use sightglass_recorder::measure::Measurements;
use sightglass_recorder::{bench_api::BenchApi, benchmark, measure::MeasureType};
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
    /// The path to the file(s) to benchmark. This accepts one or more:
    ///
    /// - `*.wasm` files: individual benchmarks that meet the requirements
    ///   outlined in `benchmarks/README.md`
    ///
    /// - `*.suite` files: a file containing a newline-delimited list of
    ///   benchmarks to execute. A `*.suite` file may contain `#`-prefixed line
    ///   comments. Relative paths are resolved against the parent directory of
    ///   the `*.suite` file.
    ///
    /// By default, this will use `benchmarks/default.suite`.
    #[structopt(
        index = 1,
        default_value = "benchmarks/default.suite",
        value_name = "FILE"
    )]
    benchmarks: Vec<BenchmarkOrSuite>,

    /// The benchmark engine(s) with which to run the benchmark.
    ///
    /// This is one or more paths to a shared library implementing the
    /// benchmarking engine specification. See `engines/wasmtime` for an example
    /// script to build an engine.
    #[structopt(long("engine"), short("e"), value_name = "PATH", empty_values = false)]
    engines: Vec<String>,

    /// Configure an engine using engine-specific flags. (For the Wasmtime
    /// engine, these can be a subset of flags from `wasmtime run --help`).
    #[structopt(long("engine-flags"), value_name = "ENGINE_FLAGS")]
    engine_flags: Option<String>,

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

    /// The type of measurement to use (cycles, insts-retired, perf-counters, noop, vtune)
    /// when recording the benchmark performance.  This option can be specified more than
    /// once if to record multiple measurements.  If no measures are specified,
    /// the "cycles" measure will be used.
    #[structopt(long = "measure", short = "m", multiple = true)]
    measures: Vec<MeasureType>,

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

    /// Benchmark only the given phase (compilation, instantiation, or
    /// execution). Benchmarks all phases if omitted.
    #[structopt(long("benchmark-phase"))]
    benchmark_phase: Option<Phase>,

    /// The significance level for confidence intervals. Typical values are 0.01
    /// and 0.05, which correspond to 99% and 95% confidence respectively. This
    /// is ignored when using `--raw` or when there aren't exactly two engines
    /// supplied.
    #[structopt(short, long, default_value = "0.01")]
    significance_level: f64,

    /// Pin all benchmark iterations in a process to a single core. See
    /// `cpu_affinity` in the `sightglass-recorder` crate for more information.
    #[structopt(long)]
    pin: bool,

    /// Keep log files after successful benchmark runs. By default, logs are
    /// only kept on failures.
    #[structopt(short = "k", long = "keep-logs")]
    keep_logs: bool,
}

impl BenchmarkCommand {
    pub fn execute(&self) -> Result<()> {
        anyhow::ensure!(self.processes > 0, "processes must be greater than zero");
        anyhow::ensure!(
            self.iterations_per_process > 0,
            "iterations-per-process must be greater than zero"
        );
        anyhow::ensure!(
            !self.engines.is_empty(),
            "must pass one or more engines to benchmark with -e/--engine"
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

        if self.pin {
            bind_to_single_core().context("attempting to pin execution to a single core")?;
        }

        let wasm_files: Vec<_> = self
            .benchmarks
            .iter()
            .flat_map(|f| f.paths())
            .map(|p| p.display().to_string())
            .collect();
        let mut all_measurements = vec![];

        for engine in &self.engines {
            let engine_path = check_engine_path(engine)?;
            log::info!("Using benchmark engine: {}", engine_path.display());
            let lib = unsafe { libloading::Library::new(&engine_path)? };
            let mut bench_api = unsafe { BenchApi::new(&lib)? };

            for wasm_file in &wasm_files {
                log::info!("Using Wasm benchmark: {}", wasm_file);

                // Use the provided --working-dir, otherwise find the Wasm file's parent directory.
                let working_dir = self.get_working_directory(&wasm_file)?;
                log::info!("Using working directory: {}", working_dir.display());

                // Read the Wasm bytes.
                let bytes = fs::read(wasm_file).context("Attempting to read Wasm bytes")?;
                log::debug!("Wasm benchmark size: {} bytes", bytes.len());

                let wasm_hash = {
                    use std::collections::hash_map::DefaultHasher;
                    use std::hash::{Hash, Hasher};
                    let mut hasher = DefaultHasher::new();
                    wasm_file.hash(&mut hasher);
                    hasher.finish()
                };

                // Create log files in temp directory
                let log_dir = std::env::temp_dir().join("sightglass-logs");
                std::fs::create_dir_all(&log_dir).context("Failed to create log directory")?;

                let stdout =
                    log_dir.join(format!("stdout-{wasm_hash:x}-{}.log", std::process::id()));
                let stderr =
                    log_dir.join(format!("stderr-{wasm_hash:x}-{}.log", std::process::id()));
                let stdin = None;

                let mut measurements = Measurements::new(this_arch(), engine, wasm_file);
                let mut measure = if self.measures.len() <= 1 {
                    let measure = self.measures.first().unwrap_or(&MeasureType::Cycles);
                    measure.build()
                } else {
                    Box::new(MultiMeasure::new(self.measures.iter().map(|m| m.build())))
                };

                // Create the bench API engine and cache it for reuse across all
                // iterations of this benchmark.
                let engine = Engine::new(
                    &mut bench_api,
                    &working_dir,
                    &stdout,
                    &stderr,
                    stdin,
                    &mut measurements,
                    &mut measure,
                    self.engine_flags.as_deref(),
                );
                let mut engine = Some(engine);

                // And if we are benchmarking just a post-compilation phase,
                // then eagerly compile the Wasm module for reuse.
                let mut module = None;
                if let Some(Phase::Instantiation | Phase::Execution) = self.benchmark_phase {
                    module = Some(engine.take().unwrap().compile(&bytes));
                }

                // Run the benchmark (compilation, instantiation, and execution) several times in
                // this process.
                for _ in 0..self.iterations_per_process {
                    match self.benchmark_phase {
                        None => {
                            let new_engine = benchmark::all(engine.take().unwrap(), &bytes)?;
                            engine = Some(new_engine);
                        }
                        Some(Phase::Compilation) => {
                            let new_engine =
                                benchmark::compilation(engine.take().unwrap(), &bytes)?;
                            engine = Some(new_engine);
                        }
                        Some(Phase::Instantiation) => {
                            let new_module = benchmark::instantiation(module.take().unwrap())?;
                            module = Some(new_module);
                        }
                        Some(Phase::Execution) => {
                            let new_module = benchmark::execution(module.take().unwrap())?;
                            module = Some(new_module);
                        }
                    }

                    let check_result = self.check_output(Path::new(wasm_file), &stdout, &stderr);

                    // Handle log cleanup based on success/failure and --keep-logs flag
                    match check_result {
                        Ok(()) => {
                            if !self.keep_logs {
                                let _ = fs::remove_file(&stdout);
                                let _ = fs::remove_file(&stderr);
                            } else {
                                log::info!(
                                    "Kept log files: {} and {}",
                                    stdout.display(),
                                    stderr.display()
                                );
                            }
                        }
                        Err(e) => {
                            // Failure: keep logs and inform user
                            eprintln!("Benchmark output check failed. Log files preserved:");
                            eprintln!("  stdout: {}", stdout.display());
                            eprintln!("  stderr: {}", stderr.display());
                            return Err(e);
                        }
                    }

                    engine
                        .as_mut()
                        .map(|e| e.measurements())
                        .or_else(|| module.as_mut().map(|m| m.measurements()))
                        .unwrap()
                        .next_iteration();
                }

                drop((engine, module));
                all_measurements.extend(measurements.finish());
            }
        }

        // If we are only benchmarking one phase then filter out any
        // measurements for other phases. These get included because we have to
        // compile at least once to measure instantiation, for example.
        if let Some(phase) = self.benchmark_phase {
            all_measurements.retain(|m| m.phase == phase);
        }

        self.write_results(&all_measurements, &mut output_file)?;
        Ok(())
    }

    /// Assert that our actual `stdout` and `stderr` match our expectations.
    fn check_output(&self, wasm_file: &Path, stdout: &Path, stderr: &Path) -> Result<()> {
        match self.benchmark_phase {
            None | Some(Phase::Execution) => {}
            // If we aren't executing the Wasm, then we won't have any actual
            // output to check.
            Some(Phase::Compilation | Phase::Instantiation) => return Ok(()),
        }

        let wasm_file_dir: PathBuf = if let Some(dir) = wasm_file.parent() {
            dir.into()
        } else {
            ".".into()
        };

        // Find the output files with benchmark file stem: e.g., `<benchmark
        // name>.stdout.expected`. This is extra prefix is needed to
        // differentiate output in directories with multiple benchmarks.
        let benchmark_name = wasm_file
            .file_stem()
            .with_context(|| "expected the benchmark file to have an extension")?
            .to_str()
            .with_context(|| "expected the benchmark file to have a printable name")?;
        let mut stdout_expected = wasm_file_dir.join(format!("{benchmark_name}.stdout.expected"));
        if !stdout_expected.exists() {
            stdout_expected = wasm_file_dir.join("default.stdout.expected");
        }
        let mut stderr_expected = wasm_file_dir.join(format!("{benchmark_name}.stderr.expected"));
        if !stderr_expected.exists() {
            stderr_expected = wasm_file_dir.join("default.stderr.expected");
        }

        compare_output_file(wasm_file, stdout, &stdout_expected)?;
        compare_output_file(wasm_file, stderr, &stderr_expected)?;
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
            let engine = check_engine_path(engine)?;

            for wasm in self.benchmarks.iter().flat_map(|s| s.paths()) {
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
                .args(
                    self.measures
                        .iter()
                        .flat_map(|m| ["--measure".to_string(), m.to_string()]),
                )
                .arg("--raw")
                .arg("--output-format")
                // Always use JSON when privately communicating with a
                // subprocess.
                .arg(Format::Json.to_string());

            if self.pin {
                command.arg("--pin");
            }

            if self.keep_logs {
                command.arg("--keep-logs");
            }

            if self.small_workloads {
                command.env("WASM_BENCH_USE_SMALL_WORKLOAD", "1");
            }

            if let Some(phase) = self.benchmark_phase {
                command.arg("--benchmark-phase").arg(phase.to_string());
            }

            if let Some(flags) = &self.engine_flags {
                command.arg(format!("--engine-flags={flags}"));
            }

            command.arg("--").arg(&wasm);

            let output = command
                .output()
                .context("failed to run benchmark subprocess")?;

            anyhow::ensure!(
                output.status.success(),
                "benchmark subprocess did not exit successfully: {}\nstderr: {}\nstdout: {}",
                output.status,
                String::from_utf8_lossy(&output.stderr),
                String::from_utf8_lossy(&output.stdout)
            );

            eprintln!(".");

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
    let effect_sizes =
        sightglass_analysis::effect_size::calculate(significance_level, measurements)?;
    let summaries = sightglass_analysis::summarize::calculate(measurements);
    sightglass_analysis::effect_size::write(
        effect_sizes,
        &summaries,
        significance_level,
        output_file,
    )
}

fn display_summaries(measurements: &[Measurement<'_>], output_file: &mut dyn Write) -> Result<()> {
    let summaries = sightglass_analysis::summarize::calculate(measurements);
    sightglass_analysis::summarize::write(summaries, output_file)
}

// Check that a passed engine path is indeed a valid path; the returned value is a path to the built
// engine's dylib.
pub fn check_engine_path(engine: &str) -> Result<PathBuf> {
    if Path::new(engine).exists() {
        log::debug!("Using engine path: {}", engine);
        Ok(PathBuf::from(engine))
    } else {
        Err(anyhow!("invalid path to engine: {}", engine))
    }
}

/// Check that the `actual` output of benchmarking `wasm` (either `stdout` or
/// `stderr`) is the same as the `expected` output.
fn compare_output_file(wasm: &Path, actual: &Path, expected: &Path) -> Result<()> {
    if expected.exists() {
        let expected_data = std::fs::read_to_string(expected)
            .with_context(|| format!("failed to read `{}`", expected.display()))?;
        let stdout_actual_data = std::fs::read_to_string(actual)
            .with_context(|| format!("failed to read `{}`", actual.display()))?;
        // Compare lines so that we ignore `\n` on *nix vs `\r\n` on Windows.
        let expected_data = expected_data.lines().collect::<Vec<_>>();
        let actual_data = stdout_actual_data.lines().collect::<Vec<_>>();
        anyhow::ensure!(
            expected_data == actual_data,
            "Actual output does not match the expected output!\n\
             * Actual output is located at `{}`\n\
             * Expected output is located at `{}`",
            actual.display(),
            expected.display(),
        );
    } else {
        log::warn!(
            "Did not find `{}` for `{}`! Cannot assert that actual \
             output ({}) matches expectation.",
            expected.display(),
            wasm.display(),
            actual.display()
        );
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
        eprintln!("=== Actual ===\n{actual}");

        let expected = r#"
compilation
  benchmarks/pulldown-cmark/benchmark.wasm
    cycles
      [696450758 740410589.60 823537015] /tmp/old_backend.so
      [688475571 710846289.20 796284592] /tmp/old_backend_2.so
      [721352134 776890922.40 933479759] /tmp/old_backend_3.so
    nanoseconds
      [239819667 254957035.80 283581244] /tmp/old_backend.so
      [237074550 244777841.50 274198271] /tmp/old_backend_2.so
      [248392822 267517235.10 321437562] /tmp/old_backend_3.so
instantiation
  benchmarks/pulldown-cmark/benchmark.wasm
    cycles
      [186145 213469.60 229974] /tmp/old_backend.so
      [200003 220099.00 308810] /tmp/old_backend_2.so
      [203474 233069.30 300269] /tmp/old_backend_3.so
    nanoseconds
      [64098 73506.90 79190] /tmp/old_backend.so
      [68870 75789.90 106337] /tmp/old_backend_2.so
      [70064 80255.30 103395] /tmp/old_backend_3.so
execution
  benchmarks/pulldown-cmark/benchmark.wasm
    cycles
      [10334150 12342413.00 14169904] /tmp/old_backend.so
      [10328193 10829803.50 12631959] /tmp/old_backend_2.so
      [10569938 11690281.50 16792916] /tmp/old_backend_3.so
    nanoseconds
      [3558517 4250053.60 4879342] /tmp/old_backend.so
      [3556483 3729210.70 4349778] /tmp/old_backend_2.so
      [3639688 4025470.30 5782529] /tmp/old_backend_3.so
"#;
        eprintln!("=== Expected ===\n{expected}");

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
        eprintln!("=== Actual ===\n{actual}");

        let expected = r#"
compilation :: cycles :: benchmarks/pulldown-cmark/benchmark.wasm

  Δ = 231879938.88 ± 5920528.32 (confidence = 95%)

  old_backend.so is 1.32x to 1.34x faster than new_backend.so!

  [889384088 935555419.78 1045075629] new_backend.so
  [688072501 703675480.90 826253416] old_backend.so

compilation :: nanoseconds :: benchmarks/pulldown-cmark/benchmark.wasm

  Δ = 79845660.57 ± 2038688.33 (confidence = 95%)

  old_backend.so is 1.32x to 1.34x faster than new_backend.so!

  [306252409 322151144.14 359863566] new_backend.so
  [236932712 242305483.57 284514295] old_backend.so

execution :: nanoseconds :: benchmarks/pulldown-cmark/benchmark.wasm

  Δ = 467229.61 ± 57708.35 (confidence = 95%)

  new_backend.so is 1.13x to 1.16x faster than old_backend.so!

  [3061587 3240065.98 4419514] new_backend.so
  [3510983 3707295.59 5811112] old_backend.so

execution :: cycles :: benchmarks/pulldown-cmark/benchmark.wasm

  Δ = 1356859.60 ± 167590.00 (confidence = 95%)

  new_backend.so is 1.13x to 1.16x faster than old_backend.so!

  [8891120 9409439.69 12834660] new_backend.so
  [10196192 10766299.29 16875960] old_backend.so

instantiation :: cycles :: benchmarks/pulldown-cmark/benchmark.wasm

  No difference in performance.

  [191466 207762.01 325810] new_backend.so
  [179617 200451.81 334016] old_backend.so

instantiation :: nanoseconds :: benchmarks/pulldown-cmark/benchmark.wasm

  No difference in performance.

  [65929 71540.70 112190] new_backend.so
  [61849 69023.59 115015] old_backend.so
"#;
        eprintln!("=== Expected ===\n{expected}");

        assert_eq!(actual.trim(), expected.trim());
        Ok(())
    }
}
