use crate::suite::BenchmarkOrSuite;
use anyhow::{Context, Result, anyhow};
use clap::Parser;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use sightglass_data::{Format, Measurement, Phase};
use sightglass_recorder::bench_api::Engine;
use sightglass_recorder::cpu_affinity::bind_to_single_core;
use sightglass_recorder::measure::Measurements;
use sightglass_recorder::measure::multi::MultiMeasure;
use sightglass_recorder::{bench_api::BenchApi, benchmark, measure::MeasureType};
use std::{
    fs,
    io::{self, BufWriter, IsTerminal},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use termcolor::{ColorChoice, NoColor, StandardStream, WriteColor};

const DEFAULT_PROCESSES: usize = 10;
const DEFAULT_ITERATIONS_PER_PROCESS: usize = 10;

#[cfg(all(target_os = "linux", feature = "callgrind"))]
mod callgrind {
    use super::*;
    use sightglass_recorder::measure::callgrind::CALLGRIND_OUT_DIR_ENV_VAR;
    use tempfile::TempDir;

    const DEFAULT_CALLGRIND_PROCESSES: usize = 3;
    const DEFAULT_CALLGRIND_ITERATIONS_PER_PROCESS: usize = 1;

    // 64KiB, 8-way associative, 64B line size.
    const CACHE_MODEL_I1: &str = "65536,8,64";
    // 64KiB, 8-way associative, 64B line size.
    const CACHE_MODEL_D1: &str = "65536,8,64";
    // 8MiB, 16-way associative, 64B line size.
    const CACHE_MODEL_LL: &str = "8388608,16,64";

    impl PreparedCommand {
        #[cfg(all(target_os = "linux", feature = "callgrind"))]
        fn with_tempdir(mut self, tempdir: tempfile::TempDir) -> Self {
            self.tempdir = Some(tempdir);
            self
        }

        #[cfg(all(target_os = "linux", feature = "callgrind"))]
        fn tempdir(&self) -> Option<&tempfile::TempDir> {
            self.tempdir.as_ref()
        }
    }

    impl BenchmarkCommand {
        pub(super) fn default_processes(&self) -> usize {
            if self.uses_callgrind() {
                DEFAULT_CALLGRIND_PROCESSES
            } else {
                DEFAULT_PROCESSES
            }
        }

        pub(super) fn default_iterations_per_process(&self) -> usize {
            if self.uses_callgrind() {
                DEFAULT_CALLGRIND_ITERATIONS_PER_PROCESS
            } else {
                DEFAULT_ITERATIONS_PER_PROCESS
            }
        }

        pub(super) fn validate(&self) -> Result<()> {
            if self.uses_callgrind() && self.measures.len() > 1 {
                anyhow::bail!(
                    "callgrind must be used by itself and cannot be combined with other measures"
                );
            }

            Ok(())
        }

        pub(super) fn should_wrap_subprocesses(&self) -> bool {
            self.uses_callgrind() && std::env::var_os(CALLGRIND_OUT_DIR_ENV_VAR).is_none()
        }

        pub(super) fn prepare_command(
            &self,
            this_exe: &Path,
            engine: &Path,
            wasm: &Path,
        ) -> Result<PreparedCommand> {
            ensure_tools_available()?;

            let callgrind_output = TempDir::new().context("failed to create callgrind tempdir")?;
            let mut prepared = PreparedCommand::new(
                Command::new("setarch"),
                "callgrind benchmark subprocess",
                "failed to run callgrind benchmark subprocess",
                "failed to read callgrind benchmark subprocess's results",
            )
            .with_tempdir(callgrind_output);
            let output_dir = prepared.tempdir().unwrap().path().to_path_buf();

            prepared
                .command
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .arg(this_arch())
                .arg("-R")
                .arg("valgrind")
                .arg("--tool=callgrind")
                .arg("--cache-sim=yes")
                .arg("--branch-sim=yes")
                .arg(format!("--I1={CACHE_MODEL_I1}"))
                .arg(format!("--D1={CACHE_MODEL_D1}"))
                .arg(format!("--LL={CACHE_MODEL_LL}"))
                .arg("--instr-atstart=no")
                .arg(format!(
                    "--callgrind-out-file={}",
                    output_dir.join("callgrind.out.%p").display()
                ))
                .arg(this_exe);
            prepared.command.env("RAYON_NUM_THREADS", "1");
            prepared.command.env(CALLGRIND_OUT_DIR_ENV_VAR, &output_dir);
            self.add_benchmark_child_args(
                &mut prepared.command,
                engine,
                wasm,
                1,
                self.iterations_per_process(),
                Format::Json,
            );

            Ok(prepared)
        }

        fn uses_callgrind(&self) -> bool {
            self.measures
                .iter()
                .any(|measure| matches!(measure, MeasureType::Callgrind))
        }
    }

    fn ensure_tools_available() -> Result<()> {
        ensure_command_succeeds(
            "valgrind",
            ["--version"],
            "callgrind measurement requires `valgrind` on PATH",
        )?;
        ensure_command_succeeds(
            "setarch",
            [this_arch(), "-R", "true"],
            "callgrind measurement requires `setarch -R` support to disable ASLR",
        )?;
        Ok(())
    }

    fn ensure_command_succeeds<I, S>(program: &str, args: I, error_message: &str) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        let status = Command::new(program)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .with_context(|| format!("{error_message}: failed to spawn `{program}`"))?;
        anyhow::ensure!(status.success(), "{error_message}");
        Ok(())
    }
}

#[cfg(not(all(target_os = "linux", feature = "callgrind")))]
mod callgrind {
    use super::*;

    impl BenchmarkCommand {
        pub(super) fn default_processes(&self) -> usize {
            DEFAULT_PROCESSES
        }

        pub(super) fn default_iterations_per_process(&self) -> usize {
            DEFAULT_ITERATIONS_PER_PROCESS
        }

        pub(super) fn validate(&self) -> Result<()> {
            Ok(())
        }

        pub(super) fn should_wrap_subprocesses(&self) -> bool {
            false
        }

        pub(super) fn prepare_command(
            &self,
            _this_exe: &Path,
            _engine: &Path,
            _wasm: &Path,
        ) -> Result<PreparedCommand> {
            unreachable!()
        }
    }
}

/// Measure compilation, instantiation, and execution of a Wasm file.
///
/// The total number of samples taken for each Wasm benchmark is `PROCESSES *
/// NUMBER_OF_ITERATIONS_PER_PROCESS`.
#[derive(Parser, Debug)]
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
    #[arg(default_value = "benchmarks/default.suite", value_name = "FILE")]
    benchmarks: Vec<BenchmarkOrSuite>,

    /// The benchmark engine(s) with which to run the benchmark.
    ///
    /// This is one or more paths to a shared library implementing the
    /// benchmarking engine specification. See `engines/wasmtime` for an example
    /// script to build an engine.
    #[arg(long = "engine", short = 'e', value_name = "PATH")]
    engines: Vec<String>,

    /// Configure an engine using engine-specific flags. (For the Wasmtime
    /// engine, these can be a subset of flags from `wasmtime run --help`).
    #[arg(
        long = "engine-flags",
        value_name = "ENGINE_FLAGS",
        allow_hyphen_values = true
    )]
    engine_flags: Option<String>,

    /// How many processes should we use for each Wasm benchmark?
    ///
    /// Defaults to `10`, unless using the `callgrind` measure, in which case the
    /// default is `3`.
    #[arg(long = "processes", value_name = "PROCESSES")]
    processes: Option<usize>,

    /// Override the "engine" name; this is useful if running experiments that might
    /// not have a differentiating engine name (e.g. if customizing the flags).
    ///
    /// If multiple engines are provided, the order of names provided here should
    /// match the order of the engines specified.
    #[arg(long = "name", short = 'n')]
    names: Option<Vec<String>>,

    /// How many times should we run a benchmark in a single process?
    ///
    /// Defaults to `10`, unless using the `callgrind` measure, in which case the
    /// default is `1`.
    #[arg(
        long = "iterations-per-process",
        value_name = "NUMBER_OF_ITERATIONS_PER_PROCESS"
    )]
    iterations_per_process: Option<usize>,

    /// Output raw data, rather than the summarized, human-readable analysis
    /// results.
    #[arg(long)]
    raw: bool,

    /// The format of the raw output data when `--raw` is used. Either 'json' or
    /// 'csv'.
    #[arg(
        short = 'f',
        long = "output-format",
        default_value = "json",
        requires = "raw"
    )]
    output_format: Format,

    /// Path to a file which will contain the output data, or nothing to print
    /// to stdout (default).
    #[arg(short = 'o', long = "output-file")]
    output_file: Option<String>,

    /// When to color the human-readable output: `auto` (the default; color only
    /// when writing to a terminal), `always`, `ansi`, or `never`.
    #[arg(
        long,
        value_parser = parse_color_choice,
        conflicts_with_all = ["raw", "output_file"],
    )]
    color: Option<ColorChoice>,

    /// The type of measurement to use (cycles, insts-retired, perf-counters,
    /// noop, vtune, callgrind) when recording benchmark performance.
    ///
    /// This option can be specified more than once to record multiple measures,
    /// except for `callgrind`, which must be used by itself.
    ///
    /// If no measures are specified, the "cycles" measure is used.
    ///
    /// `callgrind` defaults to fewer processes and iterations per process
    /// because it runs the benchmarking processes under Valgrind, which is
    /// slower but also more deterministic and less noisy.
    #[arg(long = "measure", short = 'm', action = clap::ArgAction::Append)]
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
    #[arg(long, alias = "small-workload")]
    small_workloads: bool,

    /// The directory to preopen as the benchmark working directory. If the
    /// benchmark accesses files using WASI, it will see this directory as its
    /// current working directory (i.e. `.`). If the working directory is not
    /// specified, the Wasm file's parent directory is used instead.
    #[arg(short = 'd', long = "working-dir")]
    working_dir: Option<PathBuf>,

    /// Benchmark only the given phase (compilation, instantiation, or
    /// execution). Benchmarks all phases if omitted.
    #[arg(long = "benchmark-phase")]
    benchmark_phase: Option<Phase>,

    /// The significance level for confidence intervals. Typical values are 0.01
    /// and 0.05, which correspond to 99% and 95% confidence respectively. This
    /// is ignored when using `--raw` or when there aren't exactly two engines
    /// supplied.
    #[arg(short, long, default_value = "0.01")]
    significance_level: f64,

    /// Pin all benchmark iterations in a process to a single core. See
    /// `cpu_affinity` in the `sightglass-recorder` crate for more information.
    #[arg(long)]
    pin: bool,

    /// Keep log files after successful benchmark runs. By default, logs are
    /// only kept on failures.
    #[arg(short = 'k', long = "keep-logs")]
    keep_logs: bool,
}

impl BenchmarkCommand {
    pub fn execute(&self) -> Result<()> {
        anyhow::ensure!(self.processes() > 0, "processes must be greater than zero");
        anyhow::ensure!(
            self.iterations_per_process() > 0,
            "iterations-per-process must be greater than zero"
        );
        anyhow::ensure!(
            !self.engines.is_empty(),
            "must pass one or more engines to benchmark with -e/--engine"
        );
        self.validate()?;

        if self.should_wrap_subprocesses() {
            let this_exe =
                std::env::current_exe().context("failed to get the current executable's path")?;
            return self.execute_in_subprocesses("callgrind iterations", |engine, wasm| {
                self.prepare_command(&this_exe, engine, wasm)
            });
        }

        if self.processes() == 1 {
            self.execute_in_current_process()
        } else {
            self.execute_in_multiple_processes()
        }
    }

    fn processes(&self) -> usize {
        self.processes.unwrap_or_else(|| self.default_processes())
    }

    fn iterations_per_process(&self) -> usize {
        self.iterations_per_process
            .unwrap_or_else(|| self.default_iterations_per_process())
    }

    /// Execute benchmark(s) in the provided engine(s) using the current process.
    pub fn execute_in_current_process(&self) -> Result<()> {
        let mut output_file = self.make_output_writer()?;

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

        for (i, engine_name) in self.engines.iter().enumerate() {
            let engine_path = check_engine_path(engine_name)?;
            let engine_name = self
                .names
                .as_ref()
                .and_then(|names| names.get(i).map(|s| s.as_str()))
                .unwrap_or(engine_name);
            log::info!("Using benchmark engine: {}", engine_path.display());
            let lib = unsafe { libloading::Library::new(&engine_path)? };
            let mut bench_api = unsafe { BenchApi::new(&lib)? };

            for wasm_file in &wasm_files {
                log::info!("Using Wasm benchmark: {wasm_file}");

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

                let engine = sightglass_data::Engine {
                    name: engine_name.into(),
                    flags: self.engine_flags.as_ref().map(|ef| ef.into()),
                };
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
                for _ in 0..self.iterations_per_process() {
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

            // Explicitly close the library to handle any unload errors.
            // We log errors but don't fail since measurements have been collected.
            if let Err(e) = lib.close() {
                log::warn!("Error unloading library: {e}");
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
        let this_exe =
            std::env::current_exe().context("failed to get the current executable's path")?;
        self.execute_in_subprocesses("iterations", |engine, wasm| {
            let mut prepared = PreparedCommand::new(
                Command::new(&this_exe),
                "benchmark subprocess",
                "failed to run benchmark subprocess",
                "failed to read benchmark subprocess's results",
            );
            prepared
                .command
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::inherit());
            self.add_benchmark_child_args(
                &mut prepared.command,
                engine,
                wasm,
                1,
                self.iterations_per_process(),
                Format::Json,
            );
            Ok(prepared)
        })
    }

    fn execute_in_subprocesses<F>(
        &self,
        iteration_label: &str,
        mut prepare_command: F,
    ) -> Result<()>
    where
        F: FnMut(&Path, &Path) -> Result<PreparedCommand>,
    {
        let mut output_file = self.make_output_writer()?;

        let wasm_files: Vec<_> = self.benchmarks.iter().flat_map(|b| b.paths()).collect();
        eprintln!(
            "\nRunning {} total {} ({} engines * {} benchmarks * {} processes * {} iterations per process)",
            self.engines.len()
                * wasm_files.len()
                * self.processes()
                * self.iterations_per_process(),
            iteration_label,
            self.engines.len(),
            wasm_files.len(),
            self.processes(),
            self.iterations_per_process()
        );
        eprint!("\n[Done] [Elapsed    ] [Est. Rem.  ]");

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

            for wasm in wasm_files.iter().cloned() {
                choices.push((engine.clone(), wasm, self.processes()));
            }
        }

        // Accumulated measurements from all of our subprocesses.
        let mut measurements = vec![];

        let mut i = 0;
        let n = choices.len() * self.processes();
        let start = std::time::Instant::now();

        while !choices.is_empty() {
            let index = rng.gen_range(0, choices.len());
            let (engine, wasm, procs_left) = &mut choices[index];
            let mut prepared = prepare_command(engine, wasm)?;
            let output = prepared
                .command
                .output()
                .context(prepared.failure_context)?;

            anyhow::ensure!(
                output.status.success(),
                "{} did not exit successfully: {}\nstderr: {}\nstdout: {}",
                prepared.status_label,
                output.status,
                String::from_utf8_lossy(&output.stderr),
                String::from_utf8_lossy(&output.stdout)
            );

            const PRINT_EVERY: usize = 10;
            i += 1;
            debug_assert!(i <= n);
            if i % PRINT_EVERY == 1 {
                let percent_done = (i as f64) / (n as f64) * 100.0;

                let elapsed = start.elapsed().as_secs();
                let elapsed_hours = elapsed / 60 / 60;
                let elapsed_mins = elapsed / 60 % 60;
                let elapsed_secs = elapsed % 60;

                let secs_per_proc = (elapsed as f64) / (i as f64);
                let procs_left = n - i;
                let eta = (procs_left as f64) * secs_per_proc;
                let eta = eta.round() as u64;
                let eta_hours = eta / 60 / 60;
                let eta_mins = eta / 60 % 60;
                let eta_secs = eta % 60;

                eprint!(
                    "\n\
                     [{percent_done:>3.0}%] \
                     [{elapsed_hours:02}h:{elapsed_mins:02}m:{elapsed_secs:02}s] \
                     [{eta_hours:02}h:{eta_mins:02}m:{eta_secs:02}s] ",
                )
            }
            eprint!(".");

            // Parse the subprocess's output and add its measurements to our
            // accumulation.
            measurements.extend(
                serde_json::from_slice::<Vec<Measurement<'_>>>(&output.stdout)
                    .context(prepared.result_context)?,
            );

            *procs_left -= 1;
            if *procs_left == 0 {
                choices.swap_remove(index);
            }
        }

        let elapsed = start.elapsed();
        let hours = elapsed.as_secs() / 60 / 60;
        let mins = elapsed.as_secs() / 60 % 60;
        let secs = elapsed.as_secs() % 60;
        eprintln!("\n\nFinished benchmarking in {hours:02}h:{mins:02}m:{secs:02}s");

        self.write_results(&measurements, &mut output_file)?;
        Ok(())
    }

    /// Open the output stream for results, honoring `--output-file` and
    /// `--color`.
    fn make_output_writer(&self) -> Result<Box<dyn WriteColor>> {
        if let Some(file) = self.output_file.as_ref() {
            Ok(Box::new(NoColor::new(BufWriter::new(fs::File::create(
                file,
            )?))))
        } else {
            let color = self.color.unwrap_or(ColorChoice::Auto);

            // `ColorChoice::Auto` does not itself check for a terminal, so we
            // downgrade `Auto` to `Never` when stdout is not a tty. Other choices
            // pass through unchanged.
            if color == ColorChoice::Auto && !io::stdout().is_terminal() {
                Ok(Box::new(StandardStream::stdout(ColorChoice::Never)))
            } else {
                Ok(Box::new(StandardStream::stdout(color)))
            }
        }
    }

    fn write_results(
        &self,
        measurements: &[Measurement<'_>],
        output_file: &mut dyn WriteColor,
    ) -> Result<()> {
        if self.raw {
            self.output_format.write(measurements, output_file)?;
        } else {
            // Augment the measurements with "Sum Total" measurements that sum
            // each sample's counts across all benchmarks, so that the analysis
            // reports totals in addition to per-benchmark results.
            let mut measurements = measurements.to_vec();
            measurements.extend(sum_totals(&measurements));

            if self.engines.len() == 2 {
                display_effect_size(&measurements, self.significance_level, output_file)?;
            } else {
                display_summaries(&measurements, output_file)?;
            }
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

    fn add_benchmark_child_args(
        &self,
        command: &mut Command,
        engine: &Path,
        wasm: &Path,
        processes: usize,
        iterations_per_process: usize,
        output_format: Format,
    ) {
        command
            .arg("benchmark")
            .arg("--processes")
            .arg(processes.to_string())
            .arg("--iterations-per-process")
            .arg(iterations_per_process.to_string())
            .arg("--engine")
            .arg(engine)
            .args(
                self.measures
                    .iter()
                    .flat_map(|measure| ["--measure".to_string(), measure.to_string()]),
            )
            .arg("--raw")
            .arg("--output-format")
            .arg(output_format.to_string());

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

        if let Some(flags) = self.engine_flags.as_deref() {
            command.arg(format!("--engine-flags={flags}"));
        }

        command.arg("--").arg(wasm);
    }
}

struct PreparedCommand {
    command: Command,
    #[cfg(all(target_os = "linux", feature = "callgrind"))]
    tempdir: Option<tempfile::TempDir>,
    status_label: &'static str,
    failure_context: &'static str,
    result_context: &'static str,
}

impl PreparedCommand {
    fn new(
        command: Command,
        status_label: &'static str,
        failure_context: &'static str,
        result_context: &'static str,
    ) -> Self {
        Self {
            command,
            #[cfg(all(target_os = "linux", feature = "callgrind"))]
            tempdir: None,
            status_label,
            failure_context,
            result_context,
        }
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
    output_file: &mut dyn WriteColor,
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

fn display_summaries(
    measurements: &[Measurement<'_>],
    output_file: &mut dyn WriteColor,
) -> Result<()> {
    let summaries = sightglass_analysis::summarize::calculate(measurements);
    sightglass_analysis::summarize::write(summaries, output_file)
}

/// Parse a `--color` value into a `ColorChoice`.
fn parse_color_choice(s: &str) -> Result<ColorChoice, String> {
    match s.to_ascii_lowercase().as_str() {
        "auto" => Ok(ColorChoice::Auto),
        "always" | "y" | "yes" => Ok(ColorChoice::Always),
        "ansi" => Ok(ColorChoice::AlwaysAnsi),
        "never" | "n" | "no" => Ok(ColorChoice::Never),
        _ => Err(format!(
            "invalid color choice `{s}` (expected auto, always, ansi, or never)"
        )),
    }
}

/// Sum measurement `count`s across all benchmarks for each iteration, producing
/// new "Sum Total" measurements.
///
/// That is, if the given measurements have 50 unique samples for each
/// benchmark, then this will add 50 "Sum Total" measurements, each of which is
/// the sum of, e.g., instructions retired when compiling all benchmarks on
/// their `i`th sample.
///
/// This is useful because (a) it gives us a single "top line" number for the
/// whole benchmark run, and (b) on noisy machines with high variance it can
/// (often) be the case that any individual benchmark doesn't show statistically
/// significant differences between two engines but the sum totals *do* show
/// statistically significant differences due to effetively having a greater
/// number of samples.
fn sum_totals<'a>(measurements: &[Measurement<'a>]) -> Vec<Measurement<'a>> {
    use std::borrow::Cow;
    use std::collections::BTreeMap;

    // Key by every field except `wasm` and `count`: (arch, engine, process,
    // iteration, phase, event).
    let mut totals: BTreeMap<
        (
            Cow<'a, str>,
            sightglass_data::Engine<'a>,
            u32,
            u32,
            Phase,
            Cow<'a, str>,
        ),
        u64,
    > = BTreeMap::new();

    for m in measurements {
        let key = (
            m.arch.clone(),
            m.engine.clone(),
            m.process,
            m.iteration,
            m.phase,
            m.event.clone(),
        );
        *totals.entry(key).or_insert(0) += m.count;
    }

    totals
        .into_iter()
        .map(
            |((arch, engine, process, iteration, phase, event), count)| Measurement {
                arch,
                engine,
                wasm: "Sum Total".into(),
                process,
                iteration,
                phase,
                event,
                count,
            },
        )
        .collect()
}

// Check that a passed engine path is indeed a valid path; the returned value is a path to the built
// engine's dylib.
pub fn check_engine_path(engine: &str) -> Result<PathBuf> {
    if Path::new(engine).exists() {
        log::debug!("Using engine path: {engine}");
        Ok(PathBuf::from(engine))
    } else {
        Err(anyhow!("invalid path to engine: {engine}"))
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
    fn sum_totals_sums_counts_across_benchmarks() {
        let m = |wasm: &'static str, iteration: u32, count: u64| Measurement {
            arch: "x86_64".into(),
            engine: sightglass_data::Engine {
                name: "e".into(),
                flags: None,
            },
            wasm: wasm.into(),
            process: 7,
            iteration,
            phase: Phase::Execution,
            event: "cycles".into(),
            count,
        };

        // Two benchmarks measured over two iterations, sharing every other
        // field. Each (process, iteration) sample is summed across benchmarks.
        let measurements = vec![
            m("a.wasm", 0, 10),
            m("b.wasm", 0, 100),
            m("a.wasm", 1, 20),
            m("b.wasm", 1, 200),
        ];

        let totals = sum_totals(&measurements);
        assert_eq!(totals.len(), 2);
        for t in &totals {
            assert_eq!(t.wasm, "Sum Total");
            assert_eq!(t.process, 7);
            assert_eq!(t.phase, Phase::Execution);
            assert_eq!(t.event, "cycles");
        }

        let mut by_iteration: Vec<_> = totals.iter().map(|t| (t.iteration, t.count)).collect();
        by_iteration.sort();
        assert_eq!(by_iteration, vec![(0, 110), (1, 220)]);
    }

    #[test]
    fn test_display_summaries() -> Result<()> {
        let fixture = std::fs::read("../../test/fixtures/old-backends.json")
            .context("failed to read fixture file")?;
        let measurements: Vec<Measurement<'_>> = serde_json::from_slice(&fixture)?;
        let mut output = NoColor::new(Vec::new());
        display_summaries(&measurements, &mut output)?;

        let actual = String::from_utf8(output.into_inner())?;
        eprintln!("=== Actual ===\n{actual}");

        let expected = r#"
compilation
    pulldown-cmark
        cycles
            ┌───────────┬───────────┬──────────────┬───────────┬───────────────────────┐
            │ Min       │ Max       │ Mean         │ Median    │ Engine                │
            ├───────────┼───────────┼──────────────┼───────────┼───────────────────────┤
            │ 696450758 │ 823537015 │ 740410589.60 │ 725850172 │ /tmp/old_backend.so   │
            ├───────────┼───────────┼──────────────┼───────────┼───────────────────────┤
            │ 688475571 │ 796284592 │ 710846289.20 │ 704156210 │ /tmp/old_backend_2.so │
            ├───────────┼───────────┼──────────────┼───────────┼───────────────────────┤
            │ 721352134 │ 933479759 │ 776890922.40 │ 751713121 │ /tmp/old_backend_3.so │
            └───────────┴───────────┴──────────────┴───────────┴───────────────────────┘
        nanoseconds
            ┌───────────┬───────────┬──────────────┬───────────┬───────────────────────┐
            │ Min       │ Max       │ Mean         │ Median    │ Engine                │
            ├───────────┼───────────┼──────────────┼───────────┼───────────────────────┤
            │ 239819667 │ 283581244 │ 254957035.80 │ 249943222 │ /tmp/old_backend.so   │
            ├───────────┼───────────┼──────────────┼───────────┼───────────────────────┤
            │ 237074550 │ 274198271 │ 244777841.50 │ 242474132 │ /tmp/old_backend_2.so │
            ├───────────┼───────────┼──────────────┼───────────┼───────────────────────┤
            │ 248392822 │ 321437562 │ 267517235.10 │ 258847426 │ /tmp/old_backend_3.so │
            └───────────┴───────────┴──────────────┴───────────┴───────────────────────┘
instantiation
    pulldown-cmark
        cycles
            ┌────────┬────────┬───────────┬────────┬───────────────────────┐
            │ Min    │ Max    │ Mean      │ Median │ Engine                │
            ├────────┼────────┼───────────┼────────┼───────────────────────┤
            │ 186145 │ 229974 │ 213469.60 │ 221862 │ /tmp/old_backend.so   │
            ├────────┼────────┼───────────┼────────┼───────────────────────┤
            │ 200003 │ 308810 │ 220099.00 │ 213468 │ /tmp/old_backend_2.so │
            ├────────┼────────┼───────────┼────────┼───────────────────────┤
            │ 203474 │ 300269 │ 233069.30 │ 214567 │ /tmp/old_backend_3.so │
            └────────┴────────┴───────────┴────────┴───────────────────────┘
        nanoseconds
            ┌───────┬────────┬──────────┬────────┬───────────────────────┐
            │ Min   │ Max    │ Mean     │ Median │ Engine                │
            ├───────┼────────┼──────────┼────────┼───────────────────────┤
            │ 64098 │ 79190  │ 73506.90 │ 76397  │ /tmp/old_backend.so   │
            ├───────┼────────┼──────────┼────────┼───────────────────────┤
            │ 68870 │ 106337 │ 75789.90 │ 73507  │ /tmp/old_backend_2.so │
            ├───────┼────────┼──────────┼────────┼───────────────────────┤
            │ 70064 │ 103395 │ 80255.30 │ 73884  │ /tmp/old_backend_3.so │
            └───────┴────────┴──────────┴────────┴───────────────────────┘
execution
    pulldown-cmark
        cycles
            ┌──────────┬──────────┬─────────────┬──────────┬───────────────────────┐
            │ Min      │ Max      │ Mean        │ Median   │ Engine                │
            ├──────────┼──────────┼─────────────┼──────────┼───────────────────────┤
            │ 10334150 │ 14169904 │ 12342413.00 │ 13146295 │ /tmp/old_backend.so   │
            ├──────────┼──────────┼─────────────┼──────────┼───────────────────────┤
            │ 10328193 │ 12631959 │ 10829803.50 │ 10688469 │ /tmp/old_backend_2.so │
            ├──────────┼──────────┼─────────────┼──────────┼───────────────────────┤
            │ 10569938 │ 16792916 │ 11690281.50 │ 10845308 │ /tmp/old_backend_3.so │
            └──────────┴──────────┴─────────────┴──────────┴───────────────────────┘
        nanoseconds
            ┌─────────┬─────────┬────────────┬─────────┬───────────────────────┐
            │ Min     │ Max     │ Mean       │ Median  │ Engine                │
            ├─────────┼─────────┼────────────┼─────────┼───────────────────────┤
            │ 3558517 │ 4879342 │ 4250053.60 │ 4526867 │ /tmp/old_backend.so   │
            ├─────────┼─────────┼────────────┼─────────┼───────────────────────┤
            │ 3556483 │ 4349778 │ 3729210.70 │ 3680543 │ /tmp/old_backend_2.so │
            ├─────────┼─────────┼────────────┼─────────┼───────────────────────┤
            │ 3639688 │ 5782529 │ 4025470.30 │ 3734509 │ /tmp/old_backend_3.so │
            └─────────┴─────────┴────────────┴─────────┴───────────────────────┘
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
        let mut output = NoColor::new(Vec::new());
        display_effect_size(&measurements, 0.05, &mut output)?;

        let actual = String::from_utf8(output.into_inner())?;
        eprintln!("=== Actual ===\n{actual}");

        let expected = r#"
compilation :: cycles :: pulldown-cmark

    Δ = 231879938.88 ± 5920528.32 (confidence = 95%)

    old_backend.so is 1.32x to 1.34x faster than new_backend.so!

    ┌───────────┬────────────┬──────────────┬───────────┬────────────────┐
    │ Min       │ Max        │ Mean         │ Median    │ Engine         │
    ├───────────┼────────────┼──────────────┼───────────┼────────────────┤
    │ 889384088 │ 1045075629 │ 935555419.78 │ 932321327 │ new_backend.so │
    ├───────────┼────────────┼──────────────┼───────────┼────────────────┤
    │ 688072501 │ 826253416  │ 703675480.90 │ 699614293 │ old_backend.so │
    └───────────┴────────────┴──────────────┴───────────┴────────────────┘

compilation :: nanoseconds :: pulldown-cmark

    Δ = 79845660.57 ± 2038688.33 (confidence = 95%)

    old_backend.so is 1.32x to 1.34x faster than new_backend.so!

    ┌───────────┬───────────┬──────────────┬───────────┬────────────────┐
    │ Min       │ Max       │ Mean         │ Median    │ Engine         │
    ├───────────┼───────────┼──────────────┼───────────┼────────────────┤
    │ 306252409 │ 359863566 │ 322151144.14 │ 321037510 │ new_backend.so │
    ├───────────┼───────────┼──────────────┼───────────┼────────────────┤
    │ 236932712 │ 284514295 │ 242305483.57 │ 240907043 │ old_backend.so │
    └───────────┴───────────┴──────────────┴───────────┴────────────────┘

execution :: nanoseconds :: pulldown-cmark

    Δ = 467229.61 ± 57708.35 (confidence = 95%)

    new_backend.so is 1.13x to 1.16x faster than old_backend.so!

    ┌─────────┬─────────┬────────────┬─────────┬────────────────┐
    │ Min     │ Max     │ Mean       │ Median  │ Engine         │
    ├─────────┼─────────┼────────────┼─────────┼────────────────┤
    │ 3061587 │ 4419514 │ 3240065.98 │ 3194630 │ new_backend.so │
    ├─────────┼─────────┼────────────┼─────────┼────────────────┤
    │ 3510983 │ 5811112 │ 3707295.59 │ 3673498 │ old_backend.so │
    └─────────┴─────────┴────────────┴─────────┴────────────────┘

execution :: cycles :: pulldown-cmark

    Δ = 1356859.60 ± 167590.00 (confidence = 95%)

    new_backend.so is 1.13x to 1.16x faster than old_backend.so!

    ┌──────────┬──────────┬─────────────┬──────────┬────────────────┐
    │ Min      │ Max      │ Mean        │ Median   │ Engine         │
    ├──────────┼──────────┼─────────────┼──────────┼────────────────┤
    │ 8891120  │ 12834660 │ 9409439.69  │ 9277491  │ new_backend.so │
    ├──────────┼──────────┼─────────────┼──────────┼────────────────┤
    │ 10196192 │ 16875960 │ 10766299.29 │ 10668147 │ old_backend.so │
    └──────────┴──────────┴─────────────┴──────────┴────────────────┘

instantiation :: cycles :: pulldown-cmark

    No difference in performance.

    ┌────────┬────────┬───────────┬────────┬────────────────┐
    │ Min    │ Max    │ Mean      │ Median │ Engine         │
    ├────────┼────────┼───────────┼────────┼────────────────┤
    │ 191466 │ 325810 │ 207762.01 │ 199222 │ new_backend.so │
    ├────────┼────────┼───────────┼────────┼────────────────┤
    │ 179617 │ 334016 │ 200451.81 │ 188412 │ old_backend.so │
    └────────┴────────┴───────────┴────────┴────────────────┘

instantiation :: nanoseconds :: pulldown-cmark

    No difference in performance.

    ┌───────┬────────┬──────────┬────────┬────────────────┐
    │ Min   │ Max    │ Mean     │ Median │ Engine         │
    ├───────┼────────┼──────────┼────────┼────────────────┤
    │ 65929 │ 112190 │ 71540.70 │ 68600  │ new_backend.so │
    ├───────┼────────┼──────────┼────────┼────────────────┤
    │ 61849 │ 115015 │ 69023.59 │ 64878  │ old_backend.so │
    └───────┴────────┴──────────┴────────┴────────────────┘
"#;
        eprintln!("=== Expected ===\n{expected}");

        assert_eq!(actual.trim(), expected.trim());
        Ok(())
    }

    #[cfg(all(target_os = "linux", feature = "callgrind"))]
    #[test]
    fn callgrind_must_be_exclusive() {
        let command = BenchmarkCommand {
            benchmarks: vec![],
            engines: vec!["/tmp/engine.so".into()],
            engine_flags: None,
            processes: None,
            names: None,
            iterations_per_process: None,
            raw: false,
            output_format: Format::Json,
            output_file: None,
            color: None,
            measures: vec![MeasureType::Callgrind, MeasureType::Cycles],
            small_workloads: false,
            working_dir: None,
            benchmark_phase: None,
            significance_level: 0.01,
            pin: false,
            keep_logs: false,
        };

        assert_eq!(
            command.validate().unwrap_err().to_string(),
            "callgrind must be used by itself and cannot be combined with other measures"
        );
    }
}
