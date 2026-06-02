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
use wasmparser::{
    component_types::ComponentEntityType, types::Types, Encoding, Import, Payload, TypeRef,
};

pub struct WasmBenchmark(PathBuf);

impl WasmBenchmark {
    /// Return the expected source location of a Wasm benchmark inside a benchmark Docker image.
    pub fn source() -> PathBuf {
        PathBuf::from("/benchmark.wasm")
    }

    pub fn from<P: AsRef<Path>>(path: P) -> Self {
        Self(path.as_ref().canonicalize().expect("a valid path"))
    }

    /// Verify that the Wasm file is a valid benchmark, runnable in sightglass.
    pub fn is_valid(&self) -> Result<(), ValidationError> {
        // Check that the file actually exists.
        if !self.0.exists() {
            return ValidationErrorKind::DoesNotExist.with(self);
        }

        // Check that the contents are readable.
        let bytes = match fs::read(&self.0) {
            Ok(b) => b,
            Err(_) => {
                return ValidationErrorKind::Unreadable.with(self);
            }
        };

        // Check that it contains valid Wasm, keeping the resolved type
        // information so we can inspect a component's imports below.
        let features = wasmparser::WasmFeatures::default();
        let mut validator = wasmparser::Validator::new_with_features(features);
        let types = match validator.validate_all(&bytes) {
            Ok(types) => types,
            Err(_) => return ValidationErrorKind::InvalidWasm.with(self),
        };

        // Check that it imports the `bench.start` and `bench.end` timing hooks.
        //
        // In a core module these are plain function imports. In a component
        // they are functions exported by an imported instance named `bench`.
        let component = is_component(&bytes);
        let has_hook = |field: &str| -> bool {
            if component {
                component_imports_bench_func(&types, field)
            } else {
                has_import_function(&bytes, "bench", field).unwrap()
            }
        };
        if !has_hook("start") {
            return ValidationErrorKind::MissingImport("bench.start").with(self);
        }
        if !has_hook("end") {
            return ValidationErrorKind::MissingImport("bench.end").with(self);
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
        file.write_all(b"\n")?; // Append a newline on the end.
        Ok(wat)
    }
}

impl AsRef<Path> for WasmBenchmark {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl From<WasmBenchmark> for PathBuf {
    fn from(val: WasmBenchmark) -> Self {
        val.0
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

/// Determine whether the given Wasm bytes encode a component rather than a core
/// module by inspecting the encoding in their header, which is always the first
/// payload.
fn is_component(bytes: &[u8]) -> bool {
    matches!(
        wasmparser::Parser::new(0).parse(bytes, true),
        Ok(wasmparser::Chunk::Parsed {
            payload: Payload::Version {
                encoding: Encoding::Component,
                ..
            },
            ..
        })
    )
}

/// Check whether a (validated) component imports an instance named `bench` that
/// exports a function named `field` (e.g. `start` or `end`).
fn component_imports_bench_func(types: &Types, field: &str) -> bool {
    let Some(ComponentEntityType::Instance(instance)) =
        types.component_entity_type_of_import("bench")
    else {
        return false;
    };
    matches!(
        types
            .as_ref()
            .get(instance)
            .and_then(|instance| instance.exports.get(field)),
        Some(ComponentEntityType::Func(_))
    )
}

fn has_import_function(bytes: &[u8], expected_module: &str, expected_field: &str) -> Result<bool> {
    let parser = wasmparser::Parser::new(0);
    for payload in parser.parse_all(bytes) {
        if let Payload::ImportSection(imports) = payload? {
            for imps in imports {
                let imps = imps?;
                for imp in imps {
                    if let Import {
                        module,
                        name: field,
                        ty: TypeRef::Func(_),
                    } = imp?.1
                    {
                        if module == expected_module && field == expected_field {
                            return Ok(true);
                        }
                    }
                }
            }
        }
    }
    Ok(false)
}
