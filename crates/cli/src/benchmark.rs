use anyhow::{Context, Result};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use sightglass_artifact::get_built_engine;
use sightglass_data::Format;
use sightglass_recorder::measure::Measurements;
use sightglass_recorder::{
    benchmark::{benchmark, BenchApi},
    measure::MeasureType,
};
use std::{fs, io, path::PathBuf, process::Command, process::Stdio};
use structopt::StructOpt;

/// Measure compilation, instantiation, and execution of a Wasm file.
///
/// The total number of samples taken for each Wasm benchmark is `PROCESSES *
/// NUMBER_OF_ITERATIONS_PER_PROCESS`.
#[derive(StructOpt, Debug)]
pub struct BenchmarkCommand {
    /// The benchmark engine with which to run the benchmark. This can be either the path to a
    /// shared library implementing the benchmarking engine specification or an engine reference:
    /// `[engine name]@[Git revision]?@[Git repository]?`, e.g. `wasmtime@main`.
    #[structopt(
        long("engine"),
        short("e"),
        value_name = "ENGINE-REF OR PATH",
        empty_values = false
    )]
    engines: Vec<String>,

    /// How many processes should we use for each Wasm benchmark?
    #[structopt(long = "processes", default_value = "30", value_name = "PROCESSES")]
    processes: usize,

    /// How many times should we run a benchmark in a single process?
    #[structopt(
        long = "num-iterations-per-process",
        default_value = "1",
        value_name = "NUMBER_OF_ITERATIONS_PER_PROCESS"
    )]
    iterations_per_process: usize,

    /// The format of the output data. Either 'json' or 'csv'.
    #[structopt(short = "f", long = "output-format", default_value = "json")]
    output_format: Format,

    /// The type of measurement to use (wall-cycles, perf-counters, noop) when recording the
    /// benchmark performance.
    #[structopt(long, short, default_value = "wall-cycles")]
    measure: MeasureType,

    /// The path to the Wasm file to compile.
    #[structopt(
        index = 1,
        required = true,
        value_name = "WASMFILE",
        parse(from_os_str),
        empty_values(false),
        last(true)
    )]
    wasm_files: Vec<PathBuf>,
}

impl BenchmarkCommand {
    pub fn execute(&self) -> Result<()> {
        anyhow::ensure!(self.processes > 0, "processes must be greater than zero");
        anyhow::ensure!(
            self.iterations_per_process > 0,
            "num-iterations-per-process must be greater than zero"
        );

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
            for wasm in &self.wasm_files {
                choices.push((engine, wasm, self.processes));
            }
        }

        let mut csv_header = true;

        while !choices.is_empty() {
            let index = rng.gen_range(0, choices.len());
            let (engine, wasm, procs_left) = &mut choices[index];

            let mut command = Command::new(&this_exe);
            command
                .stdin(Stdio::null())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .arg("in-process-benchmark")
                .arg("--engine")
                .arg(engine.to_string())
                .arg("--measure")
                .arg(self.measure.to_string())
                .arg("--output-format")
                .arg(self.output_format.to_string())
                .arg("--num-iterations")
                .arg(self.iterations_per_process.to_string())
                .arg(wasm);
            if csv_header {
                command.arg("--csv-header");
            }

            let status = command
                .status()
                .context("failed to spawn benchmark process")?;

            anyhow::ensure!(
                status.success(),
                "benchmark process did not exit successfully"
            );

            *procs_left -= 1;
            if *procs_left == 0 {
                choices.swap_remove(index);
            }

            // We only want to write the CSV header for the very first
            // subprocess.
            csv_header = false;
        }

        Ok(())
    }
}

/// Spawn a benchmark process.
#[derive(StructOpt, Debug)]
pub struct InProcessBenchmarkCommand {
    /// The benchmark engine with which to run the benchmark. This can be either the path to a
    /// shared library implementing the benchmarking engine specification or an engine reference:
    /// `[engine name]@[Git revision]?@[Git repository]?`, e.g. `wasmtime@main`.
    #[structopt(long("engine"), short("e"), value_name = "ENGINE-REF OR PATH")]
    engine: String,

    /// The type of measurement to use (wall-cycles, perf-counters, noop) when recording the
    /// benchmark performance.
    #[structopt(long, short, default_value = "wall-cycles")]
    measure: MeasureType,

    /// The path to the Wasm file to benchmark.
    #[structopt(
        index = 1,
        required = true,
        value_name = "WASMFILE",
        parse(from_os_str)
    )]
    wasmfile: PathBuf,

    /// The number of times to re-run the benchmark.
    #[structopt(short = "n", long = "num-iterations", default_value = "1")]
    iterations: usize,

    /// The format of the output data. Either 'json' or 'csv'.
    #[structopt(short = "f", long = "output-format", default_value = "json")]
    output_format: Format,

    /// If we are using the CSV output format, should we write a CSV header?
    ///
    /// Has no effect if the output format is not CSV.
    #[structopt(long)]
    csv_header: bool,
}

impl InProcessBenchmarkCommand {
    pub fn execute(&self) -> Result<()> {
        let engine_path = get_built_engine(&self.engine)?;
        log::info!("Using benchmark engine: {}", engine_path.display());
        let lib = libloading::Library::new(&engine_path)?;
        let mut bench_api = unsafe { BenchApi::new(&lib)? };

        log::info!("Using Wasm benchmark: {}", self.wasmfile.display());
        let bytes = fs::read(&self.wasmfile).context("Attempting to read Wasm bytes")?;
        log::debug!("Wasm benchmark size: {} bytes", bytes.len());

        let arch = this_arch();
        let engine = self.engine.to_string();
        let wasm = self.wasmfile.display().to_string();
        let mut measurements = Measurements::new(arch, &engine, &wasm);

        let mut measure = self.measure.build();

        for _ in 0..self.iterations {
            benchmark(&mut bench_api, &bytes, &mut measure, &mut measurements)?;
            measurements.next_iteration();
        }

        let measurements = measurements.finish();
        self.output_format.write(&measurements, io::stdout())
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
