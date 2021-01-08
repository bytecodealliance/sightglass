use crate::{DockerBuildArgs, Dockerfile, GitLocation};
use anyhow::{anyhow, Result};
use log;
use std::{env, fmt, fs, path::Path, path::PathBuf, str::FromStr};

// Retrieve a built engine library for running benchmarks; the returned value is a path to the built
// engine's dylib. This function will attempt to build the library if it does not yet exist.
pub fn get_built_engine(engine: &str) -> Result<PathBuf> {
    if Path::new(engine).exists() {
        log::debug!("Using already-built engine path: {}", engine);
        return Ok(PathBuf::from(engine));
    }

    // Get the path to where the known engine dylib would be if it is built, or else propagate an
    // unknown engine error.
    let engine_path = get_known_engine_path(engine)?;

    // If no file exists at the engine path, then we have to build it.
    if !engine_path.exists() {
        build_engine(engine, &engine_path)?;
        assert!(engine_path.exists());
    }

    log::debug!("Using known engine at path: {}", engine_path.display());
    Ok(engine_path)
}

/// Calculate the path to an engine library: e.g. `<user's app data
/// dir>/sightglass/wasmtime@ab1234ef/libengine.so`.
pub fn get_known_engine_path(slug: &str) -> Result<PathBuf> {
    let mut p = super::sightglass_data_dir()?;
    p.push(slug);
    p.push(get_engine_filename());
    Ok(p)
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

/// Calculate the path to the Dockerfile for building a known engine.
pub fn get_known_dockerfile_path(slug: &str) -> Result<PathBuf> {
    let mut path = PathBuf::from("."); // TODO calculate the project directory
    path.push("engines");
    path.push(slug);
    path.push("Dockerfile");
    Ok(path)
}

/// Build an engine from either a Dockerfile or a known engine.
pub fn build_engine(engine: &str, engine_path: &Path) -> Result<()> {
    // If the known engine's directory is not yet created, create it.
    let engine_dir = engine_path.parent().unwrap();
    if !engine_dir.is_dir() {
        fs::create_dir_all(&engine_dir)?;
        log::debug!("Created sightglass directory: {}", engine_dir.display());
    }

    let (dockerfile, args) = if Path::new(engine).exists() {
        (Dockerfile::from(PathBuf::from(engine)), None)
    } else {
        let engine_ref = EngineRef::from_str(engine)?;
        let dockerfile =
            Dockerfile::from(get_known_dockerfile_path(&engine_ref.engine.to_string())?);

        // Set up any additional arguments for building the library.
        let mut args = DockerBuildArgs::new();
        if let Some(revision) = &engine_ref.git.revision {
            args.set("REVISION".to_string(), revision.clone())
        }
        if let Some(repository) = &engine_ref.git.repository {
            args.set("REPOSITORY".to_string(), repository.clone())
        }

        (dockerfile, Some(args))
    };

    log::debug!("Using Dockerfile at path: {}", dockerfile);
    dockerfile.extract(format!("/{}", get_engine_filename()), engine_path, args)?;
    Ok(())
}

/// An engine-ref is a parseable description of a benchmarking engine. This crate understands how to
/// build certain well-known engines.
#[derive(Clone, Debug)]
pub struct EngineRef {
    engine: WellKnownEngine,
    git: GitLocation,
}
impl fmt::Display for EngineRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.engine)?;
        if let Some(rev) = &self.git.revision {
            write!(f, "@{}", rev)?;
        }
        if let Some(repo) = &self.git.repository {
            write!(f, "@{}", repo)?;
        }
        Ok(())
    }
}
impl FromStr for EngineRef {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split('@').collect();
        let engine = if let Some(name) = parts.get(0) {
            name.parse()?
        } else {
            return Err(anyhow!(
                "failed to parse engine-ref; the engine-ref must not be empty"
            ));
        };
        let revision = parts.get(1).map(|s| s.to_string());
        let repository = parts.get(2).map(|s| s.to_string());
        if parts.len() > 3 {
            return Err(anyhow!("an engine-ref must not have more than 3 parts: [engine name]@[revision]@[repository]"));
        }
        Ok(Self {
            engine,
            git: GitLocation {
                repository,
                revision,
            },
        })
    }
}

/// Enumerates the engines known to sightglass.
#[derive(Clone, Debug)]
pub enum WellKnownEngine {
    Wasmtime,
    V8,
}
impl fmt::Display for WellKnownEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Wasmtime => "wasmtime",
            Self::V8 => "v8",
        };
        write!(f, "{}", s)
    }
}
impl FromStr for WellKnownEngine {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "wasmtime" => Ok(Self::Wasmtime),
            "v8" => Ok(Self::V8),
            _ => Err(anyhow!("unable to parse an unknown engine: {}", s)),
        }
    }
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

    #[test]
    fn engine_ref() {
        fn roundtrip(engine_ref: &str) -> Result<()> {
            let er = EngineRef::from_str(engine_ref)?;
            if engine_ref != er.to_string() {
                Err(anyhow!("{} != {}", engine_ref, er.to_string()))
            } else {
                Ok(())
            }
        }

        // Check valid engine-refs.
        assert!(roundtrip("wasmtime").is_ok());
        assert!(roundtrip("wasmtime@1234567").is_ok());
        assert!(roundtrip("wasmtime@1234567@https://github.com/user/wasmtime").is_ok());

        // Check invalid engine-refs.
        assert!(roundtrip("other_engine").is_err());
        assert!(roundtrip("wasmtime@1234567@https://github.com/user/wasmtime@...").is_err());
    }
}
