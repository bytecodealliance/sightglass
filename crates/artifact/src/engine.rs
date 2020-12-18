use crate::{docker::DockerError, DockerBuildArgs, Dockerfile, GitLocation};
use dirs::home_dir;
use log;
use std::{fmt, fs, path::PathBuf, str::FromStr};
use thiserror::Error;

/// Abstracts the artifact that implements the Wasm benchmarking engine. This provides the
/// functionality for finding engine artifacts (i.e. shared libraries) and building them from
/// Dockerfiles; eventually it should be combined with sightglass-recorder's Engine structure.
#[derive(Debug)]
pub enum Engine {
    // An engine specified by its engine-ref: e.g. `wasmtime`, `wasmtime@8273ba32`,
    // `wasmtime@some-branch@https://github.com/user/wasmtime`
    EngineRef(EngineRef),
    // An engine specified by its location in the file system.
    LibraryPath(PathBuf),
}
impl Engine {
    /// Calculate the path to the engine library.
    pub fn path(&self) -> PathBuf {
        match self {
            Self::EngineRef(e) => Self::get_library_location(&e.to_string()),
            Self::LibraryPath(p) => p.clone(),
        }
    }

    /// Check if the engine library exists.
    pub fn exists(&self) -> bool {
        self.path().is_file()
    }

    /// If the engine is a reference, build it.
    pub fn build(&self) -> Result<(), EngineError> {
        log::info!("Building engine: {:?}", self);
        if let Self::EngineRef(e) = self {
            EngineBuilder::from(e.clone()).build()?; // TODO this is a bit backwards: we extract the engine-ref only to recreate engine
            assert!(self.exists());
            Ok(())
        } else {
            Err(EngineError::CannotBuildEnginePath)
        }
    }

    /// Calculate the default location of an engine's shared library based on a unique slug.
    fn get_library_location(slug: &str) -> PathBuf {
        let mut p = home_dir().expect("a home directory to be present");
        p.push(".sightglass");
        p.push(slug);
        if !p.is_dir() {
            fs::create_dir_all(&p).expect("to create the sightglass directory");
        }
        p.push("engine.so"); // TODO the `.so` is too platform-specific.
        p
    }
}
impl FromStr for Engine {
    type Err = EngineError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match EngineRef::from_str(s) {
            Ok(l) => Ok(Self::EngineRef(l)),
            Err(_) => Ok(Self::LibraryPath(PathBuf::from(s))),
        }
    }
}
impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EngineRef(e) => write!(f, "{}", e),
            Self::LibraryPath(p) => write!(f, "{}", p.display()),
        }
    }
}
impl From<EngineRef> for Engine {
    fn from(e: EngineRef) -> Engine {
        Engine::EngineRef(e)
    }
}
impl From<&Dockerfile> for Engine {
    fn from(e: &Dockerfile) -> Engine {
        Engine::LibraryPath(Self::get_library_location(&e.to_string()))
    }
}

/// Describes the ways an engine can be built. Either we have well-known Dockerfiles that
/// parameterized for different repositories and revisions or we build from a Dockerfile directly.
#[derive(Debug)]
pub enum EngineBuilder {
    /// The name and, optionally, a Git location path to a Dockerfile that will build a benchmark engine.
    WellKnown(EngineRef),
    /// The path to a Dockerfile that will build a benchmark engine.
    Dockerfile(Dockerfile),
}
impl EngineBuilder {
    /// Return the expected source location of a benchmark engine inside a Docker image. TODO this
    /// is too platform-specific.
    pub fn source() -> PathBuf {
        PathBuf::from("/engine.so")
    }

    /// Build an engine library using Docker.
    pub fn build(&self) -> Result<Engine, EngineError> {
        log::info!("Building engine from builder: {:?}", self);
        let engine = self.as_engine();
        let (dockerfile, args) = self.as_dockerfile();
        dockerfile.extract(Self::source(), engine.path(), args)?;
        Ok(engine)
    }

    /// Create an engine instance from this builder.
    pub fn as_engine(&self) -> Engine {
        match self {
            Self::WellKnown(location) => Engine::from(location.clone()),
            Self::Dockerfile(dockerfile) => Engine::from(dockerfile),
        }
    }

    /// Find the Dockerfile used for building this engine.
    fn as_dockerfile(&self) -> (Dockerfile, Option<DockerBuildArgs>) {
        match self {
            Self::WellKnown(location) => {
                // Find the path to the well-known Dockerfile.
                let mut path = PathBuf::from("."); // TODO calculate the project directory
                path.push("engines");
                path.push(&location.engine.to_string());
                path.push("Dockerfile");
                let dockerfile = Dockerfile::from(path);

                // Set up any additional arguments for building the library.
                let mut args = DockerBuildArgs::new();
                if let Some(revision) = &location.git.revision {
                    args.set("REVISION".to_string(), revision.clone())
                }
                if let Some(repository) = &location.git.repository {
                    args.set("REPOSITORY".to_string(), repository.clone())
                }

                (dockerfile, Some(args))
            }
            Self::Dockerfile(dockerfile) => (dockerfile.clone(), None),
        }
    }
}
impl FromStr for EngineBuilder {
    type Err = EngineError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match EngineRef::from_str(s) {
            Ok(l) => Ok(Self::WellKnown(l)),
            Err(_) => Ok(Self::Dockerfile(Dockerfile::from(PathBuf::from(s)))),
        }
    }
}
impl From<EngineRef> for EngineBuilder {
    fn from(e: EngineRef) -> EngineBuilder {
        EngineBuilder::WellKnown(e)
    }
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
    type Err = EngineError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split('@').collect();
        let engine = if let Some(name) = parts.get(0) {
            name.parse()?
        } else {
            return Err(EngineError::EmptyEngineRef);
        };
        let revision = parts.get(1).map(|s| s.to_string());
        let repository = parts.get(2).map(|s| s.to_string());
        if parts.len() > 3 {
            return Err(EngineError::TooLargeEngineRef);
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
}
impl fmt::Display for WellKnownEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Wasmtime => "wasmtime",
        };
        write!(f, "{}", s)
    }
}
impl FromStr for WellKnownEngine {
    type Err = EngineError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "wasmtime" => Ok(Self::Wasmtime),
            _ => Err(EngineError::UnknownEngine(s.to_string())),
        }
    }
}

/// Describe the ways this module can fail.
#[derive(Debug, Error)]
pub enum EngineError {
    #[error("unknown engine: {0}")]
    UnknownEngine(String),
    #[error("the engine-ref must not be empty")]
    EmptyEngineRef,
    #[error(
        "an engine-ref must not have more than 3 parts: [engine name]@[revision]@[repository]"
    )]
    TooLargeEngineRef,
    #[error("the engine is specified by a path that is not buildable; use an engine-ref instead")]
    CannotBuildEnginePath,
    #[error("failed to build engine in Docker")]
    FailedBuild(#[from] DockerError),
}
