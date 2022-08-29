//! Build a Sightglass engine using Wasmtime. Usage:
//!
//! ```
//! rustc build.rs
//! [REPOSITORY=<repo url>] [REVISION=<hash|branch|tag>] ./build [<destination dir>]
//! ```
//!
//! Note that a `hash` must be the full commit hash.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::or_fun_call)]

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

fn main() {
    // The sole CLI argument is the path at which to place the built engine library and metadata.
    let args: Vec<_> = env::args_os().collect();
    let destination_dir = match args.get(1) {
        Some(p) => Path::new(p)
            .canonicalize()
            .expect("the first parameter is not a valid directory"),
        None => env::current_dir().unwrap(),
    };

    // Collect configuration for building the engine library from environment variables.
    // - `REPOSITORY` controls the Wasmtime source to pull
    // - `REVISION` is a valid Git identifier: e.g., branch name, tag name, long or short commit
    //   hash.
    // - `BUILD_DIR` is used internally to control where the library is cloned to and built; if not
    //   present, a temporary directory is created and later removed
    let repository =
        var("REPOSITORY").unwrap_or("https://github.com/bytecodealliance/wasmtime/".into());
    let revision = var("REVISION").unwrap_or("main".into());
    let (build_dir, remove_build_dir) = if let Some(p) = env::var_os("BUILD_DIR") {
        let p = PathBuf::from(p)
            .canonicalize()
            .expect("BUILD_DIR must be a valid directory");
        (p, false)
    } else {
        (create_temp_directory(), true)
    };

    // Clone the repository at the specified revision. The sequence below is more space-efficient
    // (and thus faster) than cloning the entire repository.
    section("Retrieving the repository");
    exec(&["git", "init"], &build_dir);
    exec(&["git", "remote", "add", "origin", &repository], &build_dir);
    exec(
        &["git", "fetch", "--depth", "1", "origin", &revision],
        &build_dir,
    );
    exec(&["git", "checkout", "FETCH_HEAD"], &build_dir);
    exec(
        &["git", "submodule", "update", "--init", "--depth", "1"],
        &build_dir,
    );

    // Build the engine library.
    section("Building the engine");
    exec(
        &["cargo", "build", "--release", "-p", "wasmtime-bench-api"],
        &build_dir,
    );

    // Construct a `.build-info` file that will capture the important details a user would want to
    // know if attempting to replicate benchmark results. (The current set is not exhaustive!).
    section("Collecting metadata");
    let build_info = write_buildinfo(&build_dir, &repository, &revision);
    let build_info_contents =
        fs::read_to_string(&build_info).expect("unable to read .build-info file");
    eprintln!("{}", build_info_contents);

    // Finally, the generated files are copied to their destination and we clean up the build
    // directory.
    section("Copying files to destination");
    let from_engine_library = build_dir
        .join("target/release")
        .join(as_library_filename("wasmtime_bench_api"));
    let to_engine_library = destination_dir.join(as_library_filename("engine"));
    copy(from_engine_library, to_engine_library);
    let to_build_info = destination_dir.join(".build-info");
    copy(build_info, to_build_info);
    if remove_build_dir {
        eprintln!(
            "Removing temporary build directory: {}",
            build_dir.display()
        );
        fs::remove_dir_all(&build_dir).expect("unable to clean up temporary build directory");
    }
}

/// Print a section header for logging.
fn section(title: &str) {
    eprintln!();
    eprintln!("===== {} =====", title);
}

/// Helpful wrapper to access an environment variable as a string. `env::var` returns an error when
/// the `OsString` cannot be converted, which is not exactly what we want. This function panics if the
/// string cannot be converted but still returns an `Option` indicating if the variable was present.
fn var(key: &str) -> Option<String> {
    env::var_os(key).map(|s| {
        s.into_string()
            .expect("the given value could not be converted to UTF-8")
    })
}

/// Helpful wrapper to create a temporary directory; e.g., `/tmp/sightglass-wasmtime-build-<current
/// unix seconds>`)
fn create_temp_directory() -> PathBuf {
    let mut p = env::temp_dir();
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    p.push(format!(
        "sightglass-wasmtime-build-{}.{}",
        time.as_secs(),
        time.subsec_nanos()
    ));
    fs::create_dir(&p).expect("unable to create temporary build directory");
    eprintln!("Creating temporary build directory: {}", p.display());
    p
}

/// Execute a `command` in the `working_directory`, panicking on failure.
fn exec<P: AsRef<Path>>(command: &[&str], working_directory: P) {
    eprintln!("> {}", command.join(" "));
    let mut cmd = Command::new(command[0]);
    cmd.args(&command[1..]);
    cmd.current_dir(working_directory);
    cmd.status().expect("unable to execute command");
}

/// Same as `exec` but captures the command output.
fn exec_with_stdout<P: AsRef<Path>>(command: &[&str], working_directory: P) -> String {
    eprintln!("> {}", command.join(" "));
    let mut cmd = Command::new(command[0]);
    cmd.args(&command[1..]);
    cmd.current_dir(working_directory);
    let out = cmd.output().expect("unable to execute command");
    assert!(out.status.success());
    std::str::from_utf8(&out.stdout).unwrap().trim().to_string()
}

/// Collect system metadata used for building the Wasmtime engine and emit a `.build-info` file
/// containing key-value pairs.
fn write_buildinfo<P>(build_dir: P, repository: &str, revision: &str) -> PathBuf
where
    P: AsRef<Path>,
{
    let build_dir = build_dir.as_ref();
    let commit = exec_with_stdout(&["git", "rev-parse", "HEAD"], &build_dir);
    let datetime = exec_with_stdout(
        &["git", "show", "--no-patch", "--no-notes", "--pretty=%cI"],
        &build_dir,
    );
    let cargo = exec_with_stdout(&["cargo", "--version"], &build_dir);
    let rustc = exec_with_stdout(&["rustc", "--version"], &build_dir);
    let build_info = build_dir.join(".build-info");
    eprintln!("Writing metadata to {}:", build_info.display());
    {
        let mut file = File::create(&build_info).expect("failed to create .build-info file");
        writeln!(file, "NAME=wasmtime").unwrap();
        writeln!(file, "REPOSITORY={}", repository).unwrap();
        writeln!(file, "REVISION={}", revision).unwrap();
        writeln!(file, "_COMMIT={}", commit).unwrap();
        writeln!(file, "_COMMIT_DATETIME={}", datetime).unwrap();
        writeln!(file, "_CARGO={}", cargo).unwrap();
        writeln!(file, "_RUSTC={}", rustc).unwrap();
    }
    build_info
}

/// Helpful wrapper to copy a file.
fn copy<P: AsRef<Path>>(from: P, to: P) {
    let from = from.as_ref();
    let to = to.as_ref();
    eprintln!("Copying: {} -> {}", from.display(), to.display());
    fs::copy(from, to).expect("unable to copy file");
}

/// Calculate the library name for a sightglass library on the target operating system: e.g.,
/// `engine.dll`, `libengine.so`.
#[must_use]
pub fn as_library_filename(name: &str) -> String {
    format!(
        "{}{}{}",
        env::consts::DLL_PREFIX,
        name,
        env::consts::DLL_SUFFIX
    )
}
