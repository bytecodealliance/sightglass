//! An abstraction for Wasm benchmarks; this can verify whether the benchmark can be executed within the sightglass
//! benchmarking infrastructure.
use anyhow::Result;
use std::{fmt, fs};
use std::{
    fmt::{Display, Formatter},
    fs::File,
};
use std::{
    io::Write,
    path::{Path, PathBuf},
};
use thiserror::Error;
use wasmparser::{Import, ImportSectionEntryType, Payload};
use wasmprinter;

pub struct WasmBenchmark(PathBuf);

impl WasmBenchmark {
    pub fn from<P: AsRef<Path>>(path: P) -> Self {
        Self(path.as_ref().canonicalize().expect("a valid path"))
    }

    /// Verify that the Wasm file is a valid benchmark, runnable in sightglass.
    pub fn is_valid(&self) -> Result<(), ValidationError> {
        // Check that the file actually exists.
        if !self.0.exists() {
            return ValidationErrorKind::DoesNotExist.with(&self);
        }

        // Check that the contents are readable.
        let bytes = match fs::read(&self.0) {
            Ok(b) => b,
            Err(_) => {
                return ValidationErrorKind::Unreadable.with(&self);
            }
        };

        // Check that it contains valid Wasm.
        let mut validator = wasmparser::Validator::new();
        let mut features = wasmparser::WasmFeatures::default();
        features.simd = true;
        validator.wasm_features(features);
        if let Err(_) = validator.validate_all(&bytes) {
            return ValidationErrorKind::InvalidWasm.with(&self);
        }

        // Check that it has the expected imports/exports.
        if !has_import_function(&bytes, "bench", "start").unwrap() {
            return ValidationErrorKind::MissingImport("bench.end").with(&self);
        }
        if !has_import_function(&bytes, "bench", "end").unwrap() {
            return ValidationErrorKind::MissingImport("bench.end").with(&self);
        }

        Ok(())
    }

    /// Emit the WebAssembly Text (WAT) version of the Wasm benchmark. This will calculate a path to
    /// write to by replacing the benchmark's `.wasm` extension with `.wat`. On success, this will
    /// return the path to the written WAT.
    pub fn emit_wat(&self) -> Result<PathBuf> {
        let stem = self
            .0
            .file_stem()
            .expect("the Wasm benchmark should have a file stem (i.e. basename)");
        let mut wat = self
            .0
            .parent()
            .expect("the Wasm benchmark should have a parent directory")
            .to_path_buf();
        wat.push(format!(
            "{}.wat",
            stem.to_str()
                .expect("a valid Unicode file name for the Wasm benchmark")
        ));
        let mut file = File::create(&wat)?;
        file.write_all(wasmprinter::print_file(&self.0)?.as_bytes())?;
        file.write(&['\n' as u8])?; // Append a newline on the end.
        Ok(wat)
    }
}

impl AsRef<Path> for WasmBenchmark {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl Into<PathBuf> for WasmBenchmark {
    fn into(self) -> PathBuf {
        self.0
    }
}

#[derive(Debug, Error)]
#[error("invalid wasmfile {path}: {source}")]
pub struct ValidationError {
    path: String,
    #[source]
    source: ValidationErrorKind,
}

#[derive(Debug, Error)]
pub enum ValidationErrorKind {
    #[error("the file does not exist")]
    DoesNotExist,
    #[error("cannot read the file")]
    Unreadable,
    #[error("the file is not a valid Wasm module")]
    InvalidWasm,
    #[error("the Wasm module is missing an import: {0}")]
    MissingImport(&'static str),
}

impl ValidationErrorKind {
    fn with(self, wasmfile: &WasmBenchmark) -> Result<(), ValidationError> {
        Err(ValidationError {
            path: wasmfile.to_string(),
            source: self,
        })
    }
}

impl Display for WasmBenchmark {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

fn has_import_function(bytes: &[u8], expected_module: &str, expected_field: &str) -> Result<bool> {
    let parser = wasmparser::Parser::new(0);
    for payload in parser.parse_all(&bytes) {
        match payload? {
            Payload::ImportSection(imports) => {
                for import in imports {
                    match import? {
                        Import {
                            module,
                            field: Some(field),
                            ty: ImportSectionEntryType::Function(_),
                        } => {
                            if module == expected_module && field == expected_field {
                                return Ok(true);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    Ok(false)
}
