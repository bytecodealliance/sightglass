use anyhow::{Context, Result};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use sightglass_recorder::{
    benchmark::{benchmark, BenchApi},
    measure::MeasureType,
};
use std::str::FromStr;
use std::{fmt, io};
use std::{fs, process::Command};
use std::{path::PathBuf, process::Stdio};
use structopt::StructOpt;

/// Measure compilation, instantiation, and execution of a Wasm file.
///
/// The total number of samples taken for each Wasm benchmark is `PROCESSES *
/// SAMPLES_PER_PROCESS`.
#[derive(StructOpt, Debug)]
pub struct BenchmarkCommand {
    /// The path to the shared library implementing the benchmark API.
    #[structopt(
        short("e"),
        long("engine"),
        value_name = "ENGINE",
        parse(from_os_str),
        empty_values(false)
    )]
    engines: Vec<PathBuf>,

    /// How many processes should we use for each Wasm benchmark?
    #[structopt(long = "processes", default_value = "30", value_name = "PROCESSES")]
    processes: usize,

    /// How many samples should we take from a single process?
    #[structopt(
        long = "samples-per-process",
        default_value = "1",
        value_name = "SAMPLES_PER_PROCESS"
    )]
    samples_per_process: usize,

    /// The format of the output data. Either 'json' or 'csv'.
    #[structopt(short = "f", long = "output-format", default_value = "json")]
    output_format: OutputFormat,

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
            self.samples_per_process > 0,
            "samples-per-process must be greater than zero"
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

        while !choices.is_empty() {
            let index = rng.gen_range(0, choices.len());
            let (engine, wasm, procs_left) = &mut choices[index];

            let status = Command::new(&this_exe)
                .stdin(Stdio::null())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .arg("in-process-benchmark")
                .arg("--engine")
                .arg(engine)
                .arg("--measure")
                .arg(self.measure.to_string())
                .arg("--output-format")
                .arg(self.output_format.to_string())
                .arg(wasm)
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
        }

        Ok(())
    }
}

/// Spawn a benchmark process.
#[derive(StructOpt, Debug)]
pub struct InProcessBenchmarkCommand {
    /// The location at which to store the generated Wasm benchmark.
    #[structopt(long("engine"), short("e"), value_name = "ENGINE", parse(from_os_str))]
    engine: PathBuf,

    /// The type of measurement to use (wall-cycles, perf-counters, noop) when recording the
    /// benchmark performance.
    #[structopt(long, short, default_value = "wall-cycles")]
    measure: MeasureType,

    /// The path to the Wasm file to compile.
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
    output_format: OutputFormat,
}

impl InProcessBenchmarkCommand {
    pub fn execute(&self) -> Result<()> {
        let bytes = fs::read(&self.wasmfile).context("Attempting to read Wasm bytes")?;

        let lib = libloading::Library::new(&self.engine)?;
        let mut bench_api = unsafe { BenchApi::new(&lib)? };

        let mut measurements = Vec::with_capacity(self.iterations);
        for _ in 0..self.iterations {
            measurements.push(benchmark(&mut bench_api, &bytes, self.measure)?);
        }

        match self.output_format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string(&measurements)?);
            }
            OutputFormat::Csv => {
                let mut csv = csv::Writer::from_writer(io::stdout());
                csv.serialize(&measurements)?;
                csv.flush()?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
enum OutputFormat {
    Json,
    Csv,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Csv => write!(f, "csv"),
        }
    }
}

impl FromStr for OutputFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "csv" => Ok(OutputFormat::Csv),
            _ => Err("output format must be either 'json' or 'csv'"),
        }
    }
}
