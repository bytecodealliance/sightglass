use anyhow::Result;
use sightglass_analysis::summarize;
use sightglass_data::Format;
use std::{
    fs::File,
    io::{self, BufReader},
};
use structopt::StructOpt;

/// Summarize benchmark output; accepts raw benchmark results in `stdin` (i.e.,
/// from `sightglass-cli benchmark ...`) and prints the summarized results to
/// `stdout`.
#[derive(Debug, StructOpt)]
#[structopt(name = "summarize")]
pub struct SummarizeCommand {
    /// Path to the file(s) that will be read from, or none to indicate stdin (default).
    #[structopt(short = "f")]
    input_file: Option<Vec<String>>,

    /// The format of the input data. Either 'json' or 'csv'.
    #[structopt(short = "i", long = "input-format", default_value = "json")]
    input_format: Format,

    /// The format of the output data. Either 'json' or 'csv'; if unspecified, print the output in
    /// human-readable form.
    #[structopt(short = "o", long = "output-format")]
    output_format: Option<Format>,
}

impl SummarizeCommand {
    pub fn execute(&self) -> Result<()> {
        let measurements = if let Some(files) = self.input_file.as_ref() {
            let mut ms = Vec::new();
            for file in files {
                let reader = BufReader::new(File::open(file)?);
                ms.append(&mut self.input_format.read(reader)?);
            }
            ms
        } else {
            self.input_format.read(io::stdin())?
        };

        let summaries = summarize::calculate(&measurements);
        if let Some(output_format) = &self.output_format {
            output_format.write(&summaries, io::stdout())
        } else {
            summarize::write(summaries, &mut io::stdout())
        }
    }
}
