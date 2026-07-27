use assert_cmd::prelude::*;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

pub const DATA_DIR_ENV_VAR: &str = "SIGHTGLASS_DATA_DIR";

/// Get a `Command` for this crate's `sightglass-cli` executable.
pub fn sightglass_cli() -> Command {
    drop(env_logger::try_init());
    let mut cmd = Command::cargo_bin("sightglass-cli").unwrap();
    cmd.env(DATA_DIR_ENV_VAR, data_dir());
    cmd
}

/// Get a fresh directory for `sightglass-cli` to save its benchmark data file
/// in.
///
/// Our working directory is the crate root, so without this every non-`--raw`
/// benchmark run in the test suite would leave a `sightglass-data.csv` in the
/// source tree, and tests running in parallel would race over it. Tests that
/// assert on the data file override this with a directory of their own.
fn data_dir() -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir()
        .join("sightglass-test-data")
        .join(format!("{}-{n}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Get the path to the engine we are testing with.
pub fn test_engine() -> PathBuf {
    if let Ok(engine) = std::env::var("SIGHTGLASS_TEST_ENGINE") {
        // Use the engine specified by the environment variable. We use this to
        // cache built `libwasmtime_bench_api.so`s in CI.
        engine.into()
    } else {
        // Make sure we only ever build Wasmtime once, and don't have N threads
        // build it in parallel and race to be the one to save it onto the file
        // system.
        let project_dir = PathBuf::from("../..").canonicalize().unwrap();
        let engine_dir = project_dir.join("engines/wasmtime");
        let engine_path = engine_dir.join(sightglass_build::get_engine_filename());
        static BUILD_WASMTIME: std::sync::Once = std::sync::Once::new();
        BUILD_WASMTIME.call_once(|| {
            if engine_path.is_file() {
                // Use the already built engine library.
            } else {
                // Use this instead of `eprintln!` to avoid `cargo test`'s stdio
                // capturing.
                use std::io::Write;
                drop(writeln!(
                    std::io::stderr(),
                    "**************************************************************\n\
                 *** Building Wasmtime engine; this may take a few minutes. ***\n\
                 **************************************************************"
                ));

                // Build the `build.rs` script.
                let status = Command::new("rustc")
                    .current_dir(&engine_dir)
                    .arg("build.rs")
                    .status()
                    .expect("failed to run `rustc build.rs`");
                assert!(status.success());

                // Build the Wasmtime engine library in to the `engines/wasmtime` directory.
                let build_script_path =
                    engine_dir.join(format!("build{}", std::env::consts::EXE_SUFFIX));
                let status = Command::new(build_script_path)
                    .current_dir(&engine_dir)
                    .status()
                    .expect("failed to run `./build`");
                assert!(status.success());
            }
        });
        engine_path
    }
}

/// Get a `sightglass-cli benchmark` command that is configured to use our test engine.
pub fn sightglass_cli_benchmark() -> Command {
    let mut cmd = sightglass_cli();
    cmd.arg("benchmark").arg("--engine").arg(test_engine());
    cmd
}

/// Get the benchmark path for the benchmark with the given name.
pub fn benchmark(benchmark_name: &str) -> String {
    format!("../../benchmarks/{benchmark_name}/benchmark.wasm")
}
