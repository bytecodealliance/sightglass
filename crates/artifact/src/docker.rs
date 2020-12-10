//! Execute Docker commands as spawned processes. This module provides an abstraction, [Dockerfile], for building a
//! Wasm benchmark in a single step.
//!
//! Use the `DOCKER` environment variable to change the binary to use for this; the default is
//! `"docker"`.
use crate::wasm::WasmBenchmark;
use log::{debug, error, info};
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::string::FromUtf8Error;
use std::{convert::TryFrom, path::PathBuf};
use std::{env, fmt, io};
use thiserror::Error;

/// Represents a Dockerfile that can build a Wasm benchmark.
pub struct Dockerfile(PathBuf);

impl Dockerfile {
    /// Construct a new Dockerfile from its path.
    pub fn from(p: PathBuf) -> Self {
        Self(p.canonicalize().expect("a valid path"))
    }

    /// Find the parent directory of the Dockerfile; this is useful for determining the default context directory.
    pub fn parent_dir(&self) -> PathBuf {
        self.0
            .parent()
            .expect("to exist within a parent directory")
            .to_path_buf()
    }

    /// Build the Dockerfile and extract the file placed at `/benchmark.wasm` inside the container to `destination`. The
    /// returned [Wasmfile] contains the extracted file.
    pub fn build(&self, destination: PathBuf) -> Result<WasmBenchmark> {
        info!("Building Dockerfile: {}", self.0.display());
        let image_id = build_image(&self.0)?;
        let container_id = create_container(&image_id)?;
        let expected_build_path = PathBuf::from("/benchmark.wasm");
        copy_file_from_container(&container_id, &expected_build_path, &destination)?;
        remove_container(&container_id)?;
        remove_image(&image_id)?;
        assert!(destination.exists());
        Ok(WasmBenchmark::from(destination))
    }
}

impl Into<PathBuf> for Dockerfile {
    fn into(self) -> PathBuf {
        self.0
    }
}

pub type Result<T> = std::result::Result<T, DockerError>;

/// Build an image from a Dockerfile with the Dockerfile's parent directory as context.
pub fn build_image<P: AsRef<Path>>(dockerfile: P) -> Result<ImageId> {
    let context_dir = dockerfile
        .as_ref()
        .parent()
        .expect("a Dockerfile to exist within a parent directory");

    let mut command = Command::new(docker_binary());
    command
        .arg("build")
        .arg("-f")
        .arg(dockerfile.as_ref())
        .arg(context_dir);
    execute_and_capture_last_line(command)
}

/// Create a container from a Docker image ID.
pub fn create_container(image: &ImageId) -> Result<ContainerId> {
    let mut command = Command::new(docker_binary());
    command.arg("create").arg("-q").arg(&image.0);
    execute_and_capture_last_line(command)
}

/// Copy a file from a Docker container.
pub fn copy_file_from_container<P: AsRef<OsStr>>(
    container: &ContainerId,
    from: P,
    to: P,
) -> Result<()> {
    let mut command = Command::new(docker_binary());
    let container_location = format!("{}:{}", &container.0, from.as_ref().to_string_lossy());
    command.arg("cp").arg(container_location).arg(to);
    execute_command(command)
}

/// Remove a Docker container.
pub fn remove_container(container: &ContainerId) -> Result<()> {
    let mut command = Command::new(docker_binary());
    command.arg("rm").arg(container);
    execute_command(command)
}

/// Remove a Docker image.
pub fn remove_image(image: &ImageId) -> Result<()> {
    let mut command = Command::new(docker_binary());
    command.arg("rmi").arg(image);
    execute_command(command)
}

// Retrieve the Docker binary to use; set the `DOCKER` environment variable to change the default
// `docker`.
fn docker_binary() -> String {
    env::var("DOCKER").unwrap_or("docker".to_string())
}

// Execute a Docker command and capture the last line as a Docker ID.
fn execute_and_capture_last_line(mut command: Command) -> Result<DockerId> {
    info!("Executing: {:?}", &command);
    let mut child = command.stdout(Stdio::piped()).spawn()?;
    // TODO pipe stderr to the same place somehow.

    // Capture all printed lines to the logger and the last one as the ID.
    let reader = BufReader::new(child.stdout.take().unwrap());
    let mut lines = Vec::new();
    reader.lines().filter_map(|l| l.ok()).for_each(|l| {
        debug!("{}", &l);
        lines.push(l);
    });
    let last_line = lines.last().cloned();

    // Check that the process executed successfuly.
    let status = child.wait()?;
    if status.success() && last_line.is_some() {
        let id = DockerId::from(last_line.unwrap());
        info!("Succeeded: {}", id);
        Ok(id)
    } else {
        error!("Failed: {:?}", child);
        Err(DockerError::FailedExecution(lines.join("\n")))
    }
}

// Execute a docker command, expecting a 0 exit code.
pub fn execute_command(mut command: Command) -> Result<()> {
    info!("Executing: {:?}", &command);
    let output = command.output()?;
    if output.status.success() {
        info!("Succeeded");
        Ok(())
    } else {
        error!("Failed: {:?}", output);
        Err(DockerError::FailedExecution(
            String::from_utf8(output.stdout).unwrap(),
        ))
    }
}

/// Describe the ways this module can fail.
#[derive(Debug, Error)]
pub enum DockerError {
    #[error("failed with IO error")]
    IoError(#[from] io::Error),
    #[error("failed to execute docker command: {0}")]
    FailedExecution(String),
    #[error("failed to parse an ID")]
    FailedParsingId(#[from] FromUtf8Error),
}

pub type ImageId = DockerId;
pub type ContainerId = DockerId;

/// Container for the SHA256 digest that Docker uses for identifying objects.
#[derive(Debug)]
pub struct DockerId(String);

impl From<String> for DockerId {
    fn from(s: String) -> Self {
        DockerId(s.trim().to_string())
    }
}

impl TryFrom<Vec<u8>> for DockerId {
    type Error = DockerError;
    fn try_from(value: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Ok(Self::from(String::from_utf8(value)?))
    }
}

impl Display for DockerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl AsRef<OsStr> for DockerId {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}
