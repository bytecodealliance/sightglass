mod analyze;
mod benchmark;
mod build_benchmark;
mod build_engine;
mod validate;

use analyze::AnalyzeCommand;
use anyhow::Result;
use benchmark::BenchmarkCommand;
use build_benchmark::BuildBenchmarkCommand;
use build_engine::BuildEngineCommand;
use log::trace;
use structopt::{clap::AppSettings, StructOpt};
use validate::ValidateCommand;

/// Main entry point for CLI.
fn main() -> Result<()> {
    pretty_env_logger::init();
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
    BuildBenchmark(BuildBenchmarkCommand),
    BuildEngine(BuildEngineCommand),
    Benchmark(BenchmarkCommand),
    Validate(ValidateCommand),
    Analyze(AnalyzeCommand),
}

impl SightglassCommand {
    fn execute(&self) -> Result<()> {
        trace!("Executing command: {:?}", &self);
        match self {
            SightglassCommand::BuildBenchmark(build) => build.execute(),
            SightglassCommand::BuildEngine(build) => build.execute(),
            SightglassCommand::Benchmark(benchmark) => benchmark.execute(),
            SightglassCommand::Validate(validate) => validate.execute(),
            SightglassCommand::Analyze(analyze) => analyze.execute(),
        }
    }
}
