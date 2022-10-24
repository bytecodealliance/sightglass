mod benchmark;
mod counter;
mod effect_size;
mod fingerprint;
mod suite;
mod summarize;
mod upload;
mod validate;

use anyhow::Result;
use benchmark::BenchmarkCommand;
use effect_size::EffectSizeCommand;
use fingerprint::FingerprintCommand;
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
    EffectSize(EffectSizeCommand),
    Fingerprint(FingerprintCommand),
    Summarize(SummarizeCommand),
    UploadElastic(UploadCommand),
    Validate(ValidateCommand),
}

impl SightglassCommand {
    fn execute(&self) -> Result<()> {
        trace!("Executing command: {:?}", &self);
        match self {
            SightglassCommand::Benchmark(benchmark) => benchmark.execute(),
            SightglassCommand::EffectSize(effect_size) => effect_size.execute(),
            SightglassCommand::Fingerprint(fingerprint) => fingerprint.execute(),
            SightglassCommand::Summarize(summarize) => summarize.execute(),
            SightglassCommand::UploadElastic(upload) => upload.execute(),
            SightglassCommand::Validate(validate) => validate.execute(),
        }
    }
}
