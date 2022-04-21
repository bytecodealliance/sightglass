use anyhow::Result;
use sightglass_data::{Format, Measurement};
use sightglass_upload::upload;
use std::{
    fs::File,
    io::{self, BufReader, Read},
};
use structopt::StructOpt;

/// Upload benchmark output to server; accepts raw benchmark results in `stdin` (i.e., from
/// `sightglass-cli benchmark ...`).
#[derive(Debug, StructOpt)]
#[structopt(name = "upload")]
pub struct UploadCommand {
    /// The format of the input data. Either 'json' or 'csv'.
    #[structopt(short = "i", long = "input-format", default_value = "json")]
    input_format: Format,

    /// Path to the file that will be read from, or none to indicate stdin (default).
    #[structopt(short = "f", long = "input-file")]
    input_file: Option<String>,

    /// Setting this flag will prevent any uploading to the server.
    #[structopt(short = "d", long = "dry-run")]
    dry_run: bool,

    /// The URL of a server receiving results; this command only understands how to upload results
    /// to an ElasticSearch server; e.g., `http://localhost:9200`.
    #[structopt(index = 1, default_value = "http://localhost:9200", value_name = "URL")]
    server: String,
}

impl UploadCommand {
    pub fn execute(&self) -> Result<()> {
        let file: Box<dyn Read> = if let Some(file) = self.input_file.as_ref() {
            Box::new(BufReader::new(File::open(file)?))
        } else {
            Box::new(io::stdin())
        };
        let measurements: Vec<Measurement> = self.input_format.read(file)?;
        upload(&self.server, self.dry_run, &measurements)
    }
}
