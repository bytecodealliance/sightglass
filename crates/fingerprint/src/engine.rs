use crate::hash;
use crate::util::stringify;
use anyhow::{anyhow, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_NAME: &str = "custom";

/// Describes a WebAssembly engine.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Engine {
    /// The canonical name for the engine; e.g.:
    /// - `wasmtime-<build info hash>` for an engine with a NAME in the build info
    /// - `custom-<library hash>` for a pre-built engine at a given path
    pub name: String,
    /// The path to the engine.
    pub path: String,
    /// Describes the known configuration when building the engine using Sightglass.
    pub buildinfo: Option<String>,
}

impl Engine {
    /// Extract the build information of a Sightglass engine.
    pub fn fingerprint<P: AsRef<Path>>(library_path: P) -> Result<Self> {
        let library_path = library_path.as_ref().canonicalize()?;
        let buildinfo_path = get_buildinfo_path_from_engine_path(&library_path)?;
        let path = stringify(&library_path);

        if let Ok(buildinfo_contents) = fs::read_to_string(buildinfo_path) {
            let buildinfo_name = extract_value_from_buildinfo(&buildinfo_contents, "NAME")
                .unwrap_or(DEFAULT_NAME.into());
            let name = format!(
                "{}-{}",
                buildinfo_name,
                hash::slug(&hash::string(&buildinfo_contents))
            );
            Ok(Self {
                name,
                path,
                buildinfo: Some(buildinfo_contents),
            })
        } else {
            log::warn!(
                "No .build-info for the engine at: {}",
                &library_path.display()
            );
            Ok(Self {
                name: format!(
                    "{}-{}",
                    DEFAULT_NAME,
                    hash::slug(&hash::file(&library_path))
                ),
                path,
                buildinfo: None,
            })
        }
    }
}

/// Calculate the path to a built engine's BUILD-INFO file.
pub fn get_buildinfo_path_from_engine_path(engine_path: &Path) -> Result<PathBuf> {
    Ok(engine_path
        .parent()
        .ok_or(anyhow!("engine should have a parent directory"))?
        .join(".build-info"))
}

/// Calculate the path to a built engine's BUILD-INFO file.
pub fn extract_value_from_buildinfo(buildinfo: &str, key: &str) -> Option<String> {
    let re = Regex::new(&format!("(?m)^ *{} *= *([[:alnum:]]+) *", key)).unwrap();
    for cap in re.captures_iter(buildinfo) {
        return Some(cap[1].to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_buildinfo_file() {
        let engine_path = PathBuf::from("/some/libengine.so");
        let buildinfo_path = get_buildinfo_path_from_engine_path(&engine_path).unwrap();
        assert_eq!(buildinfo_path, PathBuf::from("/some/.build-info"));
    }

    #[test]
    fn find_name_in_buildinfo() {
        let buildinfo = r#"
        ...
        SOME_NAME_KEY=1
        ANOTHER_NAME=...
        NAME=wasmtime
        ...
        "#;
        assert_eq!(
            extract_value_from_buildinfo(buildinfo, "NAME"),
            Some("wasmtime".to_string())
        );
    }
}
