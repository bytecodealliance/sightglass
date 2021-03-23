use anyhow::Result;
use sightglass_analysis::summarize;
use sightglass_data::{Format, Measurement};
use std::{
    fs::File,
    io::{self, BufReader, Read},
};
use structopt::StructOpt;

/// Analyze benchmark output; accepts raw benchmark results in `stdin` (i.e., from `sightglass-cli
/// benchmark ...`) and prints the summarized results to `stdout`.
#[derive(Debug, StructOpt)]
#[structopt(name = "analyze")]
pub struct AnalyzeCommand {
    /// The format of the input data. Either 'json' or 'csv'.
    #[structopt(short = "i", long = "input-format", default_value = "json")]
    input_format: Format,

    /// Path to the file that will be read from, or none to indicate stdin (default).
    #[structopt(short = "f")]
    input_file: Option<String>,

    /// The format of the output data. Either 'json' or 'csv'.
    #[structopt(short = "o", long = "output-format", default_value = "json")]
    output_format: Format,
}

impl AnalyzeCommand {
    pub fn execute(&self) -> Result<()> {
        let file: Box<dyn Read> = if let Some(file) = self.input_file.as_ref() {
            Box::new(BufReader::new(File::open(file)?))
        } else {
            Box::new(io::stdin())
        };
        let measurements: Vec<Measurement> = self.input_format.read(file)?;
        let summaries = summarize(&measurements);
        self.output_format.write(&summaries, io::stdout())
    }
}
