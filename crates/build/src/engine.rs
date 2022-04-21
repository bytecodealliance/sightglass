//! Provide functions for building and listing engines.
use crate::{
    buildinfo, git,
    path::{
        get_buildinfo_path_from_engine_path, get_cache_dir, get_engine_filename,
        get_engine_path_from_buildinfo, get_engine_path_from_engine_name,
    },
    BuildInfo, Dockerfile, EngineName,
};
use anyhow::Result;
use log;
use std::{fs, path::Path, path::PathBuf, str};

/// Retrieve a built engine library for running benchmarks.
///
/// The returned value is a path to the built engine's dylib. This function will attempt to build
/// the library if it does not yet exist. `engine` can be either:
///
///  1. a path to an already-built engine library
///  2. an engine alias (see [`EngineName`])
///  3. a BUILD-INFO URI-like string (see [`BuildInfo`])
pub fn get_built_engine(engine: &str) -> Result<PathBuf> {
    // 1. a path to an already-built engine library
    if Path::new(engine).exists() {
        log::debug!("Using already-built engine path: {}", engine);
        return Ok(PathBuf::from(engine));
    }

    // 2. an engine alias
    let engine_name: Result<EngineName, _> = engine.parse();
    if let Ok(engine_name) = engine_name {
        let path = get_engine_path_from_engine_name(&engine_name)?;
        log::debug!(
            "Using already-built engine path from alias {}: {}",
            engine_name,
            path.display()
        );
        return Ok(path);
    }

    // 3. a BUILD-INFO URI-like string. Get the path to where the known engine dylib would be if it
    // is built, or else propagate an unknown engine error.
    let buildinfo = BuildInfo::parse_uri(engine)?;
    let engine_path = get_engine_path_from_buildinfo(&buildinfo)?;

    // If no file exists at the engine path, then we have to build it.
    if !engine_path.exists() {
        build_engine(&buildinfo, &engine_path)?;
        assert!(engine_path.exists());
    } else if is_build_stale(&engine_path)? {
        log::warn!(
            "The library built with this BUILD-INFO ({}) is stale",
            buildinfo
        );
    }

    log::debug!("Using known engine at path: {}", engine_path.display());
    Ok(engine_path)
}

/// Build an engine from either a Dockerfile or a known engine.
pub fn build_engine(cli_buildinfo: &BuildInfo, engine_path: &Path) -> Result<()> {
    // If the known engine's directory is not yet created, create it.
    let engine_dir = engine_path.parent().unwrap();
    if !engine_dir.is_dir() {
        fs::create_dir_all(&engine_dir)?;
        log::debug!("Created sightglass directory: {}", engine_dir.display());
    }

    // Find the Dockerfile to build with.
    let engine_name = cli_buildinfo
        .get("NAME")
        .expect("BUILDINFO must have a name");
    let dockerfile = Dockerfile::from_known_engine(engine_name)?;

    // Build the image and extract both the library and the .build-info file.
    log::debug!("Using Dockerfile at path: {}", dockerfile);
    let container_engine_path = format!("/{}", get_engine_filename());
    let container_buildinfo_path = format!("/{}", buildinfo::DEFAULT_FILE_NAME);
    let buildinfo_path = &Path::join(&engine_dir, buildinfo::DEFAULT_FILE_NAME);
    let files = [
        (container_engine_path, engine_path),
        (container_buildinfo_path, buildinfo_path),
    ];
    dockerfile.extract(&files, Some(cli_buildinfo))?;
    Ok(())
}

/// List all known engines in the cache folder.
pub fn list_engines<'a>() -> Result<Vec<(EngineName<'a>, PathBuf)>> {
    let sightglass_data_dir = get_cache_dir()?;
    let mut engines = vec![];
    for entry in fs::read_dir(sightglass_data_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let engine_dir = entry.path();
            match EngineName::from_cache_directory(engine_dir.as_path()) {
                Ok(name) => {
                    let engine_path = Path::join(&engine_dir, get_engine_filename());
                    engines.push((name, engine_path));
                }
                Err(err) => log::warn!("Invalid engine found at {}: {}", engine_dir.display(), err),
            }
        }
    }
    Ok(engines)
}

/// Check if the built engine is on a revision that may have been updated since the engine was
/// built. E.g., if `REVISION` is set to a branch, the branch may have received new commits.
fn is_build_stale(engine_path: &Path) -> Result<bool> {
    let buildinfo = BuildInfo::parse_file(&get_buildinfo_path_from_engine_path(engine_path)?)?;
    let repository = buildinfo
        .get("REPOSITORY")
        .expect("BUILDINFO must have a REPOSITORY value");
    let revision = buildinfo
        .get("REVISION")
        .expect("BUILDINFO must have a REVISION value");
    let commit = git::resolve_to_commit(repository, revision)?;
    if let Some(built_commit) = buildinfo.get("COMMIT") {
        Ok(commit == built_commit)
    } else {
        Ok(false)
    }
}
