mod benchmark;
mod build;
mod validate;

use anyhow::Result;
use benchmark::BenchmarkCommand;
use build::BuildCommand;
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
    Build(BuildCommand),
    Benchmark(BenchmarkCommand),
    Validate(ValidateCommand),
}

impl SightglassCommand {
    fn execute(&self) -> Result<()> {
        trace!("Executing command: {:?}", &self);
        match self {
            SightglassCommand::Build(build) => build.execute(),
            SightglassCommand::Benchmark(benchmark) => benchmark.execute(),
            SightglassCommand::Validate(validate) => validate.execute(),
        }
    }
}
