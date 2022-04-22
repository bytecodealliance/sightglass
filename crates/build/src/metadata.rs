//! TODO this module is largely unimplemented
//!
//! Data structures for metadata associated with an artifact:
//!  - [GitSource] describes the Git metadata from which an artifact was built
//!  - [BuildSystem] describes the build system on which an artifact was built
//!  - [BuildInfo] combines [BuildSystem] with the time the artifact was built
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, time::SystemTime};

/// Describes the Git metadata from which an artifact was built.
#[derive(Debug, Serialize, Deserialize)]
pub struct GitSource {
    repository: String,
    branch: String,
    commit: String,
}

impl GitSource {
    pub fn from<P: AsRef<Path>>(_path: P) -> Option<Self> {
        None // TODO use git2 crate
    }
}

/// Describes when and where an artifact was built.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildInfo {
    system: BuildSystem,
    time: Option<SystemTime>,
}

impl BuildInfo {
    pub fn from<P: AsRef<Path>>(path: P) -> Self {
        let system = BuildSystem::new();
        let time = fs::metadata(path).ok().map(|m| m.modified().ok()).flatten();
        Self { time, system }
    }
}

/// Describes the system on which an artifact was built.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildSystem {
    name: Option<String>,
    cpu: Option<Cpu>,
    memory: Option<Memory>,
}

impl BuildSystem {
    pub fn new() -> Self {
        Self {
            name: None,
            cpu: None,
            memory: None,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Cpu;
#[derive(Debug, Serialize, Deserialize)]
pub struct Memory;
