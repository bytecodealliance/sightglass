mod artifact;
mod docker;
mod engine;
mod git;
mod metadata;
mod wasm;

use anyhow::Context;
use std::path::PathBuf;

pub use artifact::Artifact;
pub use docker::{DockerBuildArgs, Dockerfile};
pub use engine::*;
pub use git::GitLocation;
pub use metadata::{BuildInfo, BuildSystem, GitSource};
pub use wasm::WasmBenchmark;

/// Get the local directory where sightglass stores cached data.
pub fn sightglass_data_dir() -> anyhow::Result<PathBuf> {
    let mut p = dirs::data_local_dir().ok_or_else(|| {
        anyhow::anyhow!(
            "missing an application data folder for storing sightglass engines; e.g. \
             /home/.../.local/share, C:\\Users\\...\\AppData\\Local"
        )
    })?;
    p.push("sightglass");
    Ok(p)
}

/// Clean up and remove any cached data.
pub fn clean() -> anyhow::Result<()> {
    let sightglass_data_dir = sightglass_data_dir()?;
    if !sightglass_data_dir.is_dir() {
        return Ok(());
    }

    log::info!(
        "Recursively removing sightglass cached data in {}",
        sightglass_data_dir.display()
    );
    std::fs::remove_dir_all(&sightglass_data_dir).with_context(|| {
        format!(
            "failed to recursively remove {}",
            sightglass_data_dir.display(),
        )
    })?;

    Ok(())
}
