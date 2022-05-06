//! Provide functions for constructing and converting paths to various build artifacts.
use crate::{buildinfo, BuildInfo, EngineName};
use anyhow::{anyhow, Context, Result};
use std::{
    env,
    path::{Path, PathBuf},
};

/// A reference to the Sightglass repository top-level directory. This is preserved by the
/// `build.rs` script and will not work if Sightglass is used on machines other than where it was
/// built. It is, however, quite useful for locating the engine Dockerfiles.
pub const SIGHTGLASS_PROJECT_DIRECTORY: &'static str = env!("SIGHTGLASS_PROJECT_DIRECTORY");

/// Get the local directory where sightglass stores cached data.
pub fn get_cache_dir() -> Result<PathBuf> {
    if let Ok(path) = env::var("SIGHTGLASS_CACHE_DIR") {
        return Ok(PathBuf::from(path)
            .canonicalize()
            .context("invalid SIGHTGLASS_CACHE_DIR")?);
    }

    let mut p = dirs::data_local_dir().ok_or_else(|| {
        anyhow::anyhow!(
            "missing an application data folder for storing sightglass engines; e.g. \
             /home/.../.local/share, C:\\Users\\...\\AppData\\Local"
        )
    })?;
    p.push("sightglass");
    Ok(p)
}

/// Calculate the path to an engine library using the name printed by `list-engines`. E.g.:
///
/// ```
/// # use sightglass_build::path::get_engine_path_from_engine_name;
/// # use sightglass_build::EngineName;
/// let name: EngineName = "wasmtime-1234567".parse().unwrap();
/// let path = get_engine_path_from_engine_name(&name).unwrap();
/// assert!(path.ends_with("wasmtime-1234567/libengine.so"));
/// ```
pub fn get_engine_path_from_engine_name(engine_name: &EngineName) -> Result<PathBuf> {
    Ok(get_cache_dir()?
        .join(engine_name.to_string())
        .join(get_engine_filename()))
}

/// Calculate the path to an engine library using [BuildInfo]: e.g.:
///
/// ```
/// # use sightglass_build::path::get_engine_path_from_buildinfo;
/// # use sightglass_build::BuildInfo;
/// let buildinfo = BuildInfo::parse_uri("wasmtime?REVISION=1234567").unwrap();
/// let path = get_engine_path_from_buildinfo(&buildinfo).unwrap();
/// assert!(path.ends_with("wasmtime-9c91cc9c/libengine.so"));
/// ```
pub fn get_engine_path_from_buildinfo(buildinfo: &BuildInfo) -> Result<PathBuf> {
    let engine_name = EngineName::from_buildinfo(buildinfo)?;
    get_engine_path_from_engine_name(&engine_name)
}

/// Calculate the path to an engine library using a build-info string: e.g., `wasmtime` -> `<user's
/// app data dir>/sightglass/wasmtime-ab1234ef/libengine.so`.
pub fn get_known_engine_path(uri: &str) -> Result<PathBuf> {
    let buildinfo = BuildInfo::parse_uri(uri)?;
    get_engine_path_from_buildinfo(&buildinfo)
}

/// Calculate the path to a built engine's BUILD-INFO file. E.g.:
///
/// ```
/// # use sightglass_build::path::get_buildinfo_path_from_engine_path;
/// # use std::path::PathBuf;
/// let engine_path = PathBuf::from("/some/libengine.so");
/// let buildinfo_path = get_buildinfo_path_from_engine_path(&engine_path).unwrap();
/// assert_eq!(buildinfo_path, PathBuf::from("/some/.build-info"));
/// ```
pub fn get_buildinfo_path_from_engine_path(engine_path: &Path) -> Result<PathBuf> {
    Ok(engine_path
        .parent()
        .ok_or(anyhow!("engine should have a parent directory"))?
        .join(buildinfo::DEFAULT_FILE_NAME))
}

/// Calculate the library name for a sightglass library on the target operating system: e.g.
/// `engine.dll`, `libengine.so`.
pub fn get_engine_filename() -> String {
    format!(
        "{}engine{}",
        env::consts::DLL_PREFIX,
        env::consts::DLL_SUFFIX
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn engine_filename() {
        assert_eq!("libengine.so", get_engine_filename());
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn engine_filename() {
        assert_eq!("engine.dll", get_engine_filename());
    }
}
