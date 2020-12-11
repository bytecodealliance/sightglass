use anyhow::{Context, Result};
use log::trace;
use sightglass_artifact::{Dockerfile, WasmBenchmark};
use sightglass_recorder::{benchmark::benchmark, measure::MeasureType};
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

/// Main entry point for CLI.
fn main() -> Result<()> {
    let command = SightglassCommand::from_args();
    command.execute()?;
    Ok(())
}

/// The sightglass benchmark runner.
#[derive(StructOpt, Debug)]
#[structopt(
    version = env!("CARGO_PKG_VERSION"),
    global_settings = &[
        AppSettings::VersionlessSubcommands,
        AppSettings::ColoredHelp
    ],
)]
enum SightglassCommand {
    /// Build a Wasm benchmark from a Dockerfile.
    Build(BuildCommand),
    /// Check that a Wasm benchmark is runnable in this tool.
    Validate(ValidateCommand),
    /// Measure the compilation, instantiation, and execution of a Wasm file.
    Benchmark(BenchmarkCommand),
}

#[derive(StructOpt, Debug)]
#[structopt(name = "build")]
struct BuildCommand {
    /// The location at which to store the generated Wasm benchmark.
    #[structopt(long, short, value_name = "WASMFILE", parse(from_os_str))]
    destination: Option<PathBuf>,

    /// If enabled, emit a WebAssembly Text (WAT) version of the Wasm benchmark in the same
    /// directory as the `destination` with the `.wat` suffix; e.g. if `destination` is
    /// `/usr/src/benchmark.wasm`, the WAT file will be created at `/usr/src/benchmark.wat`.
    #[structopt(long, short = "w")]
    emit_wat: bool,

    /// The path to a Dockerfile that will build a WebAssembly benchmark module.
    #[structopt(
        index = 1,
        required = true,
        value_name = "DOCKERFILE",
        parse(from_os_str)
    )]
    dockerfile: PathBuf,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "validate")]
struct ValidateCommand {
    /// The path to the WebAssembly benchmark module; this file should import `bench.start` and
    /// `bench.end`.
    #[structopt(
        index = 1,
        required = true,
        value_name = "WASMFILE",
        parse(from_os_str)
    )]
    benchmark: PathBuf,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "benchmark")]
struct BenchmarkCommand {
    /// The location at which to store the generated Wasm benchmark.
    #[structopt(long, short, value_name = "ENGINE")]
    engine: OsString,

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
}

impl SightglassCommand {
    fn execute(&self) -> Result<()> {
        pretty_env_logger::init();
        trace!("Executing command: {:?}", &self);
        match self {
            SightglassCommand::Build(c) => {
                let dockerfile = Dockerfile::from(c.dockerfile.clone());
                let destination = match c.destination.clone() {
                    None => dockerfile.parent_dir().join("benchmark.wasm"),
                    Some(p) => p,
                };
                let wasmfile = dockerfile.build(destination)?;
                if c.emit_wat {
                    wasmfile.emit_wat()?;
                }
                validate(wasmfile)
            }
            SightglassCommand::Benchmark(c) => {
                let bytes = fs::read(&c.wasmfile).context("Attempting to read Wasm bytes")?;
                let measurement = benchmark(bytes, &c.engine, c.measure)?;
                println!("{}", serde_json::to_string(&measurement)?);
            }
            SightglassCommand::Validate(c) => validate(WasmBenchmark::from(&c.benchmark)),
        }
        Ok(())
    }
}

/// Helper function for printing the validation results of a Wasm benchmark.
fn validate(benchmark: WasmBenchmark) {
    match benchmark.is_valid() {
        Ok(_) => println!("VALID: {}", benchmark),
        Err(e) => println!("INVALID: {}", e),
    }
}
