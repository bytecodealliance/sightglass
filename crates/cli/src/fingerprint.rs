use anyhow::Result;
use clap::Parser;
use sightglass_data::Format;
use sightglass_fingerprint::{Benchmark, Engine, Machine};
use std::{io, path::PathBuf};

/// Gather information about the current machine, a Wasm benchmark, or a Wasm
/// engine and print the results to `stdout`.
#[derive(Debug, Parser)]
#[command(name = "fingerprint")]
pub struct FingerprintCommand {
    /// The kind of item to fingerprint. One of: 'benchmark', 'engine', 'machine'.
    #[arg(short = 'k', long = "kind")]
    kind: Kind,

    /// The format of the output data. Either 'json' or 'csv'.
    #[arg(short = 'o', long = "output-format", default_value = "json")]
    output_format: Format,

    /// The optional path to the file to fingerprint; not all kinds
    /// fingerprinting require a file (e.g., `--kind machine`).
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,
}

impl FingerprintCommand {
    pub fn execute(&self) -> Result<()> {
        let out = io::stdout();
        match self.kind {
            Kind::Machine => self.output_format.write_one(Machine::fingerprint()?, out),
            Kind::Engine => {
                let file = self.file.as_ref().expect("an engine file must be passed");
                self.output_format
                    .write_one(Engine::fingerprint(file)?, out)
            }
            Kind::Benchmark => {
                let file = self.file.as_ref().expect("a benchmark file must be passed");
                self.output_format
                    .write_one(Benchmark::fingerprint(file)?, out)
            }
        }
    }
}

/// The kinds of Sightglass items that can be fingerprinted.
#[derive(Clone, Debug)]
enum Kind {
    Machine,
    Engine,
    Benchmark,
}

impl std::str::FromStr for Kind {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "machine" => Ok(Kind::Machine),
            "engine" => Ok(Kind::Engine),
            "benchmark" => Ok(Kind::Benchmark),
            _ => Err("output format must be one of: 'machine', 'engine', 'benchmark'"),
        }
    }
}
