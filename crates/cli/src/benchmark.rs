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

    /// The format of the output data. Either 'json' or 'csv'.
    #[structopt(short = "f", long = "output-format", default_value = "json")]
    output_format: Format,

    /// Path to a file which will contain the output data, or nothing to print to stdout (default).
    #[structopt(short = "o", long = "output-file")]
    output_file: Option<String>,

    /// The type of measurement to use (wall-cycles, perf-counters, noop) when recording the
    /// benchmark performance.
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

    /// The directory to preopen as the benchmark working directory. If the benchmark accesses
    /// files using WASI, it will see this directory as its current working directory (i.e. `.`). If
    /// the working directory is not specified, the Wasm file's parent directory is used instead.
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

        for engine in &self.engines {
            let engine_path = get_built_engine(engine)?;
            log::info!("Using benchmark engine: {}", engine_path.display());
            let lib = libloading::Library::new(&engine_path)?;
            let mut bench_api = unsafe { BenchApi::new(&lib)? };

            for wasm_file in &self.wasm_files {
                log::info!("Using Wasm benchmark: {}", wasm_file.display());

                // Use the provided --working-dir, otherwise find the Wasm file's parent directory.
                let working_dir = self.get_working_directory(&wasm_file)?;
                log::info!("Using working directory: {}", working_dir.display());

                // Read the Wasm bytes.
                let bytes = fs::read(&wasm_file).context("Attempting to read Wasm bytes")?;
                log::debug!("Wasm benchmark size: {} bytes", bytes.len());

                let wasm = wasm_file.display().to_string();
                let mut measurements = Measurements::new(this_arch(), engine, &wasm);
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

                    self.check_output(wasm_file, stdout, stderr)?;
                    measurements.next_iteration();
                }

                let measurements = measurements.finish();
                self.output_format.write(&measurements, &mut output_file)?
            }
        }
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
            let stdout_expected_data = std::fs::read(&stdout_expected)
                .with_context(|| format!("failed to read `{}`", stdout_expected.display()))?;
            let stdout_actual_data = std::fs::read(stdout)
                .with_context(|| format!("failed to read `{}`", stdout.display()))?;
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
            let stderr_expected_data = std::fs::read(&stderr_expected)
                .with_context(|| format!("failed to read `{}`", stderr_expected.display()))?;
            let stderr_actual_data = std::fs::read(stderr)
                .with_context(|| format!("failed to read `{}`", stderr.display()))?;
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
                .arg("--output-format")
                .arg(self.output_format.to_string())
                .arg(&wasm);
            if self.small_workloads {
                command.env("WASM_BENCH_USE_SMALL_WORKLOAD", "1");
            }

            let output = command
                .output()
                .context("failed to run benchmark subprocess")?;

            anyhow::ensure!(
                output.status.success(),
                "benchmark subprocess did not exit successfully"
            );

            // Parse the subprocess's output and add its measurements to our
            // accumulation.
            match self.output_format {
                Format::Json => {
                    measurements.extend(
                        serde_json::from_slice::<Vec<Measurement<'_>>>(&output.stdout)
                            .context("failed to read benchmakr subprocess's results")?,
                    );
                }
                Format::Csv { .. } => {
                    let mut reader = csv::Reader::from_reader(&output.stdout[..]);
                    for measurement in reader.deserialize() {
                        let measurement =
                            measurement.context("failed to deserialize CSV measurement record")?;
                        measurements.push(measurement);
                    }
                }
            };

            *procs_left -= 1;
            if *procs_left == 0 {
                choices.swap_remove(index);
            }
        }

        self.output_format.write(&measurements, &mut output_file)?;
        Ok(())
    }

    /// Determine the working directory in which to run the benchmark using:
    /// - first, any directory specified with `--working-dir`
    /// - then, the parent directory of the Wasm file
    /// - and if all else fails, the current working directory of the process.
    fn get_working_directory(&self, wasm_file: &PathBuf) -> Result<PathBuf> {
        let working_dir = if let Some(dir) = self.working_dir.clone() {
            dir
        } else if let Some(dir) = wasm_file.parent() {
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
