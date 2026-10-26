//! Build a Sightglass engine using Wasmtime. Usage:
//!
//! ```
//! rustc build.rs
//! ./build [--repository <url>] [--revision <hash|branch|tag>] [<destination dir>]
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

const DEFAULT_REPOSITORY: &str = "https://github.com/bytecodealliance/wasmtime.git";
const DEFAULT_REVISION: &str = "main";

const USAGE: &str = "\
Build a Wasmtime benchmarking engine for Sightglass.

Usage:
    build [OPTIONS] [<destination dir>]

Options:
    --repository <url|path>        The Wasmtime repository to build.
                                   [default: https://github.com/bytecodealliance/wasmtime.git]
    --revision <hash|branch|tag>   The revision of that repository to build; a
                                   hash must be the full commit hash.
                                   [default: main]
    --build-dir <dir>              Where to clone and build Wasmtime. Reused
                                   across runs, and not deleted afterwards.
                                   [default: a temporary directory]
    -h, --help                     Print this message.

Positional arguments:
    <destination dir>              Where to place the built engine library and
                                   its `.build-info` metadata. [default: .]
";

fn main() {
    let config = Config::from_args_and_env();

    // The destination and build directories may not exist yet, so create them
    // before canonicalizing, which requires an existing path.
    let destination_dir = make_dir(&config.destination);
    let (build_dir, remove_build_dir) = match &config.build_dir {
        Some(p) => (make_dir(p), false),
        None => (create_temp_directory(), true),
    };
    let repository = config.repository.as_str();
    let revision = config.revision.as_str();

    // Clone the repository at the specified revision. The sequence below is more space-efficient
    // (and thus faster) than cloning the entire repository.
    section("Retrieving the repository");
    exec(&["git", "init"], &[], &build_dir);
    exec(
        &["git", "remote", "add", "origin", repository],
        &[],
        &build_dir,
    );
    exec(
        &["git", "fetch", "--depth", "1", "origin", revision],
        &[],
        &build_dir,
    );
    exec(&["git", "checkout", "FETCH_HEAD"], &[], &build_dir);
    exec(
        &["git", "submodule", "update", "--init", "--depth", "1"],
        &[],
        &build_dir,
    );

    // Build the engine library.
    section("Building the engine");
    exec(
        &["cargo", "build", "--release", "-p", "wasmtime-bench-api"],
        // Avoid codegen layout noise across commits due to functions "randomly"
        // being inlined or not across builds of different Wasmtime commits
        // based on whether they are in the same CGU or not.
        //
        // We do *not* also enable LTO (or, equivalently, use Wasmtime's
        // `--profile fastest-runtime`) because this would enable more "random"
        // (across different Wasmtime commits) inlining noise between
        // crates. With CGUs=1 and LTO disabled, the inlining boundary is very
        // clear and related directly to the code we write and not our
        // dependencies: anything within a crate is a candidate for inlining
        // into function in that same crate, unless the function is marked
        // `#[inline]` (or, in some cases, is generic) in which case it can be
        // inlined across crates.
        //
        // Minimizing codegen layout noise across builds of mostly-the-same
        // Wasmtime commits is important so that when we compare the performance
        // of, say, the latest `HEAD` to `HEAD^`, we are not, as much as
        // possible, measuring changes due to LLVM's heuristics that could just
        // as easily happen with *any* commit on top of `HEAD^` and isn't
        // actually related to what `HEAD` itself is doing.
        &[("CARGO_PROFILE_RELEASE_CODEGEN_UNITS", "1")],
        &build_dir,
    );

    // Construct a `.build-info` file that will capture the important details a user would want to
    // know if attempting to replicate benchmark results. (The current set is not exhaustive!).
    section("Collecting metadata");
    let build_info = write_buildinfo(&build_dir, repository, revision);
    let build_info_contents =
        fs::read_to_string(&build_info).expect("unable to read .build-info file");
    eprintln!("{build_info_contents}");

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
    eprintln!("===== {title} =====");
}

/// The configuration for a build, assembled from CLI flags, environment variables, and defaults,
/// in that order of precedence.
struct Config {
    repository: String,
    revision: String,
    destination: PathBuf,
    build_dir: Option<PathBuf>,
}

impl Config {
    /// Parse the configuration from this process's CLI arguments and environment, exiting the
    /// process on a usage error or on `--help`.
    fn from_args_and_env() -> Self {
        let mut repository = None;
        let mut revision = None;
        let mut build_dir = None;
        let mut destination = None;

        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            // Accept both `--flag value` and `--flag=value`.
            let (flag, inline_value) = match arg.split_once('=') {
                Some((flag, value)) => (flag.to_string(), Some(value.to_string())),
                None => (arg.clone(), None),
            };
            let mut value = || match inline_value.clone().or_else(|| args.next()) {
                Some(v) => v,
                None => usage_error(&format!("{flag} requires a value")),
            };
            match flag.as_str() {
                "--repository" => repository = Some(value()),
                "--revision" => revision = Some(value()),
                "--build-dir" => build_dir = Some(PathBuf::from(value())),
                "-h" | "--help" => {
                    print!("{USAGE}");
                    std::process::exit(0);
                }
                _ if arg.starts_with('-') => usage_error(&format!("unknown flag: {arg}")),
                _ if destination.is_some() => {
                    usage_error("only one destination directory may be given")
                }
                _ => destination = Some(PathBuf::from(arg)),
            }
        }

        Self {
            repository: repository
                .or_else(|| deprecated_var("REPOSITORY", "--repository"))
                .unwrap_or(DEFAULT_REPOSITORY.into()),
            revision: revision
                .or_else(|| deprecated_var("REVISION", "--revision"))
                .unwrap_or(DEFAULT_REVISION.into()),
            destination: destination.unwrap_or(env::current_dir().unwrap()),
            build_dir: build_dir
                .or_else(|| deprecated_var("BUILD_DIR", "--build-dir").map(PathBuf::from)),
        }
    }
}

/// Print the usage message along with `message` and exit unsuccessfully.
fn usage_error(message: &str) -> ! {
    eprint!("{USAGE}");
    eprintln!();
    eprintln!("error: {message}");
    std::process::exit(1);
}

/// Read a deprecated environment variable, warning when it is present so that users migrate to
/// `flag`.
///
/// Panics if the value is not valid UTF-8.
fn deprecated_var(key: &str, flag: &str) -> Option<String> {
    let value = env::var_os(key)?.into_string().unwrap_or_else(|_| {
        // `rustc build.rs` compiles this as edition 2015, where a lone `panic!` argument is not a
        // format string, so `key` must be passed separately.
        panic!("the value of {} could not be converted to UTF-8", key);
    });
    eprintln!("warning: the {key} environment variable is deprecated; use {flag} instead");
    Some(value)
}

/// Create `dir` and any missing parents, returning its canonical path.
fn make_dir(dir: &Path) -> PathBuf {
    fs::create_dir_all(dir)
        .unwrap_or_else(|e| panic!("unable to create directory {}: {e}", dir.display()));
    dir.canonicalize()
        .unwrap_or_else(|e| panic!("unable to canonicalize {}: {e}", dir.display()))
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
fn exec<P: AsRef<Path>>(command: &[&str], env: &[(&str, &str)], working_directory: P) {
    eprintln!("> {}", command.join(" "));
    let mut cmd = Command::new(command[0]);
    cmd.args(&command[1..]);
    cmd.envs(env.iter().copied());
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
    let commit = exec_with_stdout(&["git", "rev-parse", "HEAD"], build_dir);
    let datetime = exec_with_stdout(
        &["git", "show", "--no-patch", "--no-notes", "--pretty=%cI"],
        build_dir,
    );
    let cargo = exec_with_stdout(&["cargo", "--version"], build_dir);
    let rustc = exec_with_stdout(&["rustc", "--version"], build_dir);
    let build_info = build_dir.join(".build-info");
    eprintln!("Writing metadata to {}:", build_info.display());
    {
        let mut file = File::create(&build_info).expect("failed to create .build-info file");
        writeln!(file, "NAME=wasmtime").unwrap();
        writeln!(file, "REPOSITORY={repository}").unwrap();
        writeln!(file, "REVISION={revision}").unwrap();
        writeln!(file, "_COMMIT={commit}").unwrap();
        writeln!(file, "_COMMIT_DATETIME={datetime}").unwrap();
        writeln!(file, "_CARGO={cargo}").unwrap();
        writeln!(file, "_RUSTC={rustc}").unwrap();
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
