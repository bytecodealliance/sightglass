mod benchmark;
mod effect_size;
mod fingerprint;
mod summarize;
mod validate;

use anyhow::Result;
use benchmark::BenchmarkCommand;
use effect_size::EffectSizeCommand;
use fingerprint::FingerprintCommand;
use log::trace;
use structopt::{clap::AppSettings, StructOpt};
use summarize::SummarizeCommand;
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
    Validate(ValidateCommand),
    Summarize(SummarizeCommand),
    EffectSize(EffectSizeCommand),
    Fingerprint(FingerprintCommand),
}

impl SightglassCommand {
    fn execute(&self) -> Result<()> {
        trace!("Executing command: {:?}", &self);
        match self {
            SightglassCommand::Benchmark(benchmark) => benchmark.execute(),
            SightglassCommand::Validate(validate) => validate.execute(),
            SightglassCommand::Summarize(summarize) => summarize.execute(),
            SightglassCommand::EffectSize(effect_size) => effect_size.execute(),
            SightglassCommand::Fingerprint(fingerprint) => fingerprint.execute(),
        }
    }
}
