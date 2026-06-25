use anyhow::{Context, Result};
use clap::Parser;
use sightglass_data::{Format, Measurement};
use sightglass_upload::{upload, upload_package, MeasurementPackage};
use std::{
    fs::File,
    io::{self, BufReader, Read},
};

/// Upload benchmark output to an ElasticSearch server; accepts raw benchmark
/// results in `stdin` (i.e., from `sightglass-cli benchmark ...`).
#[derive(Debug, Parser)]
#[command(name = "upload-elastic")]
pub struct UploadCommand {
    /// The format of the input data. Either 'json' or 'csv'.
    #[arg(short = 'i', long = "input-format", default_value = "json")]
    input_format: Format,

    /// Path to the file that will be read from, or none to indicate stdin
    /// (default).
    #[arg(short = 'f', long = "input-file")]
    input_file: Option<String>,

    /// The URL of a server receiving results; this command only understands how
    /// to upload results to an ElasticSearch server; e.g.,
    /// `http://localhost:9200`.
    #[arg(default_value = "http://localhost:9200", value_name = "URL")]
    server: String,

    /// Setting this flag will prevent any uploading to the server. Instead,
    /// the command will emit a JSON "package" to stdout that can be used to
    /// upload at a later time, see `--from-package`.
    #[arg(short = 'd', long = "dry-run")]
    dry_run: bool,

    /// Path to a file containing a package of measurements and fingerprint data
    /// to be uploaded. If this is set, `--input-file` and `--input-format` are
    /// ignored.
    #[arg(short = 'p', long = "from-package")]
    from_package: Option<String>,

    /// The number of measurements to upload together; this can speed up the
    /// upload. Defaults to `2000`.
    #[arg(short = 'b', long = "batch-size", default_value = "2000")]
    batch_size: usize,
}

impl UploadCommand {
    pub fn execute(&self) -> Result<()> {
        if let Some(file) = &self.from_package {
            let reader =
                BufReader::new(File::open(file).context("unable to open --from-package path")?);
            let package: MeasurementPackage =
                serde_json::from_reader(reader).context("unable to parse --from-package JSON")?;
            upload_package(&self.server, self.batch_size, self.dry_run, package)
        } else {
            let file: Box<dyn Read> = if let Some(file) = self.input_file.as_ref() {
                Box::new(BufReader::new(
                    File::open(file).context("unable to open --input-file")?,
                ))
            } else {
                Box::new(io::stdin())
            };
            let measurements: Vec<Measurement> = self.input_format.read(file)?;
            upload(&self.server, self.batch_size, self.dry_run, measurements)
        }
    }
}
