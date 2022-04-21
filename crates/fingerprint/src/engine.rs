use crate::util::stringify;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sightglass_build::{hash, path::get_buildinfo_path_from_engine_path, BuildInfo, Dockerfile};
use std::{fs, path::Path};

/// Describes a WebAssembly engine.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Engine {
    /// The canonical name for the engine; e.g.:
    /// - `wasmtime@<commit>` for an engine built using Sightglass with the given commit
    /// - `custom-<hash>` for a pre-built engine at a given path
    pub name: String,
    /// The path to the engine.
    pub path: String,
    /// Describes how to rebuild the engine using Sightglass, if this method was used.
    pub rebuild: Option<String>,
    /// Describes the known configuration when building the engine using Sightglass, if this method
    /// was used.
    pub buildinfo: Option<String>,
}

impl Engine {
    /// Extract the build information of a Sightglass engine.
    pub fn fingerprint<P: AsRef<Path>>(library_path: P) -> Result<Self> {
        let library_path = library_path.as_ref().canonicalize()?;
        let buildinfo_path = get_buildinfo_path_from_engine_path(&library_path)?;

        if let Ok(buildinfo_contents) = fs::read_to_string(buildinfo_path) {
            // Construct a rebuild string from the information stored in the .build-info file. We
            // only use the configuration that differs from the defaults in the Dockerfile for
            // brevity and human-readability.
            let buildinfo = BuildInfo::parse_file_string(&buildinfo_contents)?;
            let name = buildinfo
                .name()
                .ok_or(anyhow!(".build-info must have a valid NAME value"))?
                .to_owned();
            let dockerfile = Dockerfile::from_known_engine(&name)?;
            let buildinfo_defaults = dockerfile.default_buildinfo()?;
            let mut diffed_buildinfo = buildinfo.diff(buildinfo_defaults);

            // Also, we want to override the REVISION with the actually-built commit. REVISION may
            // have been a branch, e.g., that has new commits in it and we want a reproducible
            // build.
            diffed_buildinfo.set(
                "REVISION",
                buildinfo
                    .get("_COMMIT")
                    .expect(".build-info should contain a _COMMIT value"),
            );

            Ok(Self {
                name,
                path: stringify(library_path),
                rebuild: Some(format!("sightglass-cli build-engine {}", diffed_buildinfo)),
                buildinfo: Some(buildinfo_contents),
            })
        } else {
            log::warn!(
                "No .build-info for the engine at: {}",
                &library_path.display()
            );
            Ok(Self {
                name: format!("custom-{}", hash::slug(&hash::file(&library_path))),
                path: stringify(library_path),
                rebuild: None,
                buildinfo: None,
            })
        }
    }
}
