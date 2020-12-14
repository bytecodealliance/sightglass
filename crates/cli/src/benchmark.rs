use anyhow::{Context, Result};
use sightglass_recorder::{benchmark::benchmark, measure::MeasureType};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

/// Measure the compilation, instantiation, and execution of a Wasm file.
#[derive(StructOpt, Debug)]
#[structopt(name = "benchmark")]
pub struct BenchmarkCommand {
    /// The location at which to store the generated Wasm benchmark.
    #[structopt(long, short, value_name = "ENGINE", parse(from_os_str))]
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

    /// The format of the output data. Either 'json' or 'csv'.
    #[structopt(short = "f", long = "output-format", default_value = "json")]
    output_format: OutputFormat,
}

impl BenchmarkCommand {
    pub fn execute(&self) -> Result<()> {
        let bytes = fs::read(&self.wasmfile).context("Attempting to read Wasm bytes")?;
        let measurement = benchmark(bytes, &self.engine, self.measure)?;
        match self.output_format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string(&measurement)?);
            }
            OutputFormat::Csv => {
                let mut csv = csv::Writer::from_writer(io::stdout());
                csv.serialize(&measurement)?;
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
