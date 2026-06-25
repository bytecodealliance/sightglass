//! Describe the serialization formats used in sightglass data.
use anyhow::Result;
use core::fmt;
use csv::ReaderBuilder;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    io::{Read, Write},
    str::FromStr,
};

/// Describes the input/output formats for the data structures in the `sightglass-data` crate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    /// The JSON format.
    Json,
    /// The CSV format.
    Csv {
        /// Whether to include a header row when reading or writing.
        headers: bool,
    },
}

impl Format {
    /// Construct a CSV formatter; allows setting the `headers` parameter more easily.
    pub fn csv(headers: bool) -> Self {
        Self::Csv { headers }
    }

    /// Read a list of `T` using the selected format.
    pub fn read<T, R>(&self, reader: R) -> Result<Vec<T>>
    where
        R: Read + Sized,
        T: DeserializeOwned,
    {
        Ok(match self {
            Format::Json => serde_json::from_reader(reader)?,
            Format::Csv { headers } => {
                let mut reader = ReaderBuilder::new()
                    .has_headers(*headers)
                    .from_reader(reader);
                reader.deserialize().map(|r| r.unwrap()).collect()
            }
        })
    }

    /// Write a list of `T` using the selected format.
    pub fn write<T, W>(&self, objects: &[T], writer: W) -> Result<()>
    where
        T: Serialize,
        W: Write + Sized,
    {
        match self {
            Format::Json => serde_json::to_writer(writer, objects)?,
            Format::Csv { headers } => {
                let mut csv = csv::WriterBuilder::new()
                    .has_headers(*headers)
                    .from_writer(writer);
                for o in objects {
                    csv.serialize(o)?;
                }
                csv.flush()?;
            }
        };
        Ok(())
    }

    /// Write a single `T` using the selected format.
    pub fn write_one<T, W>(&self, object: T, writer: W) -> Result<()>
    where
        T: Serialize,
        W: Write + Sized,
    {
        match self {
            Format::Json => serde_json::to_writer(writer, &object)?,
            Format::Csv { headers } => {
                let mut csv = csv::WriterBuilder::new()
                    .has_headers(*headers)
                    .from_writer(writer);
                csv.serialize(&object)?;
                csv.flush()?;
            }
        };
        Ok(())
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Format::Json => write!(f, "json"),
            Format::Csv { .. } => write!(f, "csv"),
        }
    }
}

impl FromStr for Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "json" => Ok(Format::Json),
            "csv" => Ok(Format::Csv { headers: true }),
            _ => Err("output format must be either 'json' or 'csv'"),
        }
    }
}
