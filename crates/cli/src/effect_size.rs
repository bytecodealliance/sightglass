use anyhow::Result;
use sightglass_analysis::{effect_size, summarize};
use sightglass_data::Format;
use std::{
    fs::File,
    io::{self, BufReader},
};
use structopt::StructOpt;

/// Calculate the effect size (and associated confidence interval) between the
/// results for two different engines.
#[derive(Debug, StructOpt)]
#[structopt(name = "effect-size")]
pub struct EffectSizeCommand {
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

    /// The significance level for the confidence interval. Typical values are
    /// 0.01 and 0.05, which correspond to 99% and 95% confidence respectively.
    #[structopt(short, long, default_value = "0.01")]
    significance_level: f64,
}

impl EffectSizeCommand {
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

        let effects = effect_size::calculate(self.significance_level, &measurements)?;
        if let Some(output_format) = &self.output_format {
            output_format.write(&effects, io::stdout())
        } else {
            let summaries = summarize::calculate(&measurements);
            effect_size::write(
                effects,
                &summaries,
                self.significance_level,
                &mut io::stdout(),
            )
        }
    }
}
