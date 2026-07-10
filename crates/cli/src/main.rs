mod benchmark;
mod clean;
mod effect_size;
mod fingerprint;
mod pca_metrics;
mod report;
mod suite;
mod summarize;
mod upload;
mod validate;

use anyhow::Result;
use benchmark::BenchmarkCommand;
use clap::Parser;
use clean::CleanCommand;
use effect_size::EffectSizeCommand;
use fingerprint::FingerprintCommand;
use log::trace;
use pca_metrics::PcaMetricsCommand;
use report::ReportCommand;
use std::io::{self, IsTerminal};
use summarize::SummarizeCommand;
use termcolor::{ColorChoice, StandardStream};
use upload::UploadCommand;
use validate::ValidateCommand;

/// A stdout writer for human-readable output. Colors are only emitted when
/// stdout is a terminal.
pub(crate) fn stdout_writer() -> StandardStream {
    let choice = if io::stdout().is_terminal() {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    };
    StandardStream::stdout(choice)
}

/// Main entry point for CLI.
fn main() -> Result<()> {
    pretty_env_logger::init();
    let command = SightglassCommand::parse();
    command.execute()?;
    Ok(())
}

/// The sightglass benchmark runner.
#[derive(Parser, Debug)]
#[command(version, propagate_version = true)]
enum SightglassCommand {
    Benchmark(BenchmarkCommand),
    Clean(CleanCommand),
    EffectSize(EffectSizeCommand),
    Fingerprint(FingerprintCommand),
    PcaMetrics(PcaMetricsCommand),
    Report(ReportCommand),
    Summarize(SummarizeCommand),
    UploadElastic(UploadCommand),
    Validate(ValidateCommand),
}

impl SightglassCommand {
    fn execute(&self) -> Result<()> {
        trace!("Executing command: {:?}", &self);
        match self {
            SightglassCommand::Benchmark(benchmark) => benchmark.execute(),
            SightglassCommand::Clean(clean) => clean.execute(),
            SightglassCommand::EffectSize(effect_size) => effect_size.execute(),
            SightglassCommand::Fingerprint(fingerprint) => fingerprint.execute(),
            SightglassCommand::PcaMetrics(metrics) => metrics.execute(),
            SightglassCommand::Report(report) => report.execute(),
            SightglassCommand::Summarize(summarize) => summarize.execute(),
            SightglassCommand::UploadElastic(upload) => upload.execute(),
            SightglassCommand::Validate(validate) => validate.execute(),
        }
    }
}
