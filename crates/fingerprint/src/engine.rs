use crate::hash;
use crate::util::to_string_lossy;
use anyhow::{anyhow, Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_NAME: &str = "custom";

/// Describes a WebAssembly engine.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Engine {
    /// A unique identifier for the engine; one of:
    /// - `<engine name>-<build info hash>` for an engine with a NAME in the build info
    /// - `custom-<library hash>` for a pre-built engine at a given path (no build info)
    pub id: String,
    /// The name of the engine, if available in the build info.
    pub name: Option<String>,
    /// When the engine was at this state (the commit date), if available in the build info.
    pub datetime: Option<String>,
    /// The path to the engine.
    pub path: String,
    /// Describes the known configuration when building the engine using Sightglass.
    pub buildinfo: Option<String>,
}

impl Engine {
    /// Extract the build information of a Sightglass engine.
    pub fn fingerprint<P: AsRef<Path>>(library_path: P) -> Result<Self> {
        let library_path = library_path
            .as_ref()
            .canonicalize()
            .context("unable to get absolute path of engine")?;
        let buildinfo_path = get_buildinfo_path_from_engine_path(&library_path)?;
        let path = to_string_lossy(&library_path);

        if let Ok(buildinfo_contents) = fs::read_to_string(buildinfo_path) {
            let name = extract_value_from_buildinfo(&buildinfo_contents, "NAME");
            let datetime = extract_value_from_buildinfo(&buildinfo_contents, "_COMMIT_DATETIME");
            let id = format!(
                "{}-{}",
                name.as_ref().unwrap_or(&DEFAULT_NAME.to_string()),
                hash::slug(&hash::string(&buildinfo_contents))
            );
            Ok(Self {
                id,
                name,
                datetime,
                path,
                buildinfo: Some(buildinfo_contents),
            })
        } else {
            log::warn!(
                "No .build-info for the engine at: {}",
                &library_path.display()
            );
            let id = format!(
                "{}-{}",
                DEFAULT_NAME,
                hash::slug(&hash::file(&library_path))
            );
            Ok(Self {
                id,
                name: None,
                datetime: None,
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
    let re = Regex::new(&format!("(?m)^ *{key} *= *([[[:alnum:]]-_:]+) *")).unwrap();
    if let Some(cap) = re.captures_iter(buildinfo).next() {
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
    #[test]
    fn find_datetime_in_buildinfo() {
        let buildinfo = r#"
        ...
        _COMMIT_DATETIME=2022-06-14T12:48:15-07:00
        ...
        "#;
        assert_eq!(
            extract_value_from_buildinfo(buildinfo, "_COMMIT_DATETIME"),
            Some("2022-06-14T12:48:15-07:00".to_string())
        );
    }
}
