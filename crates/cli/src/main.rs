mod benchmark;
mod build_benchmark;
mod build_engine;
mod clean;
mod effect_size;
mod fingerprint;
mod list_engines;
mod summarize;
mod upload;
mod validate;

use anyhow::Result;
use benchmark::BenchmarkCommand;
use build_benchmark::BuildBenchmarkCommand;
use build_engine::BuildEngineCommand;
use clean::CleanCommand;
use effect_size::EffectSizeCommand;
use fingerprint::FingerprintCommand;
use list_engines::ListEnginesCommand;
use log::trace;
use structopt::{clap::AppSettings, StructOpt};
use summarize::SummarizeCommand;
use upload::UploadCommand;
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
    Benchmark(BenchmarkCommand),
    BuildBenchmark(BuildBenchmarkCommand),
    BuildEngine(BuildEngineCommand),
    Clean(CleanCommand),
    EffectSize(EffectSizeCommand),
    Fingerprint(FingerprintCommand),
    ListEngines(ListEnginesCommand),
    Summarize(SummarizeCommand),
    Upload(UploadCommand),
    Validate(ValidateCommand),
}

impl SightglassCommand {
    fn execute(&self) -> Result<()> {
        trace!("Executing command: {:?}", &self);
        match self {
            SightglassCommand::Benchmark(benchmark) => benchmark.execute(),
            SightglassCommand::BuildBenchmark(build) => build.execute(),
            SightglassCommand::BuildEngine(build) => build.execute(),
            SightglassCommand::Clean(clean) => clean.execute(),
            SightglassCommand::EffectSize(effect_size) => effect_size.execute(),
            SightglassCommand::Fingerprint(fingerprint) => fingerprint.execute(),
            SightglassCommand::ListEngines(list_engines) => list_engines.execute(),
            SightglassCommand::Summarize(summarize) => summarize.execute(),
            SightglassCommand::Upload(upload) => upload.execute(),
            SightglassCommand::Validate(validate) => validate.execute(),
        }
    }
}
