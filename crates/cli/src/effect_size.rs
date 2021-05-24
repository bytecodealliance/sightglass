use anyhow::Result;
use sightglass_analysis::effect_size;
use sightglass_data::{Format, Measurement};
use std::{
    fs::File,
    io::{self, BufReader, Read},
};
use structopt::StructOpt;

/// Calculate the effect size (and associated confidence interval) between the
/// results for two different engines.
#[derive(Debug, StructOpt)]
#[structopt(name = "effect-size")]
pub struct EffectSizeCommand {
    /// The format of the input data. Either 'json' or 'csv'.
    #[structopt(short = "i", long = "input-format", default_value = "json")]
    input_format: Format,

    /// Path to the file that will be read from, or none to indicate stdin (default).
    #[structopt(short = "f")]
    input_file: Option<String>,

    /// The format of the output data. Either 'json' or 'csv'.
    #[structopt(short = "o", long = "output-format", default_value = "json")]
    output_format: Format,

    /// The significance level for the confidence interval. Typical values are
    /// 0.01 and 0.05, which correspond to 99% and 95% confidence respectively.
    #[structopt(short, long, default_value = "0.01")]
    significance_level: f64,
}

impl EffectSizeCommand {
    pub fn execute(&self) -> Result<()> {
        let file: Box<dyn Read> = if let Some(file) = self.input_file.as_ref() {
            Box::new(BufReader::new(File::open(file)?))
        } else {
            Box::new(io::stdin())
        };
        let measurements: Vec<Measurement> = self.input_format.read(file)?;
        let effects = effect_size(self.significance_level, &measurements)?;
        self.output_format.write(&effects, io::stdout())
    }
}
