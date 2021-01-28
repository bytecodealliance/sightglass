use anyhow::Result;
use sightglass_analysis::summarize;
use sightglass_data::{Format, Measurement};
use std::io;
use structopt::StructOpt;

/// Analyze benchmark output; accepts raw benchmark results in `stdin` (i.e., from `sightglass-cli
/// benchmark ...`) and prints the summarized results to `stdout`.
#[derive(Debug, StructOpt)]
#[structopt(name = "analyze")]
pub struct AnalyzeCommand {
    /// The format of the input data. Either 'json' or 'csv'.
    #[structopt(short = "i", long = "input-format", default_value = "json")]
    input_format: Format,

    /// The format of the output data. Either 'json' or 'csv'.
    #[structopt(short = "o", long = "output-format", default_value = "json")]
    output_format: Format,
}

impl AnalyzeCommand {
    pub fn execute(&self) -> Result<()> {
        let measurements: Vec<Measurement> = self.input_format.read(io::stdin())?;
        let summaries = summarize(&measurements);
        self.output_format.write(&summaries, io::stdout())
    }
}
