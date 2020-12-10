//! Describe an Wasm benchmark artifact.
pub use crate::{BuildInfo, Dockerfile, GitSource, WasmBenchmark};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// This structure contains all of the metadata describing a Wasm benchmark artifact.
#[derive(Debug, Serialize, Deserialize)]
pub struct Artifact {
    name: String,
    artifacts: Vec<PathBuf>,
    build: BuildInfo,
    vcs: Option<GitSource>,
}

impl Artifact {
    /// Describe an artifact using from its Dockerfile and already-build Wasm benchmark.
    #[allow(dead_code)]
    pub fn from(dockerfile: Dockerfile, benchmark: WasmBenchmark) -> Self {
        let dir = dockerfile.parent_dir();
        Self {
            name: directory_name(&dir),
            vcs: GitSource::from(&dir),
            build: BuildInfo::from(&benchmark),
            artifacts: vec![dockerfile.into(), benchmark.into()],
        }
    }
}

/// Figure out a benchmark name from the directory it was built in.
fn directory_name<P: AsRef<Path>>(dir: P) -> String {
    let dir = dir.as_ref();
    assert!(dir.is_dir());
    dir.file_name()
        .expect("directory path to have a final component")
        .to_owned()
        .to_string_lossy()
        .to_string()
}
