use assert_cmd::prelude::*;
use std::path::PathBuf;
use std::process::Command;

/// Get a `Command` for this crate's `sightglass-cli` executable.
pub fn sightglass_cli() -> Command {
    drop(env_logger::try_init());
    Command::cargo_bin("sightglass-cli").unwrap()
}

/// Get the path to the engine we are testing with.
pub fn test_engine() -> PathBuf {
    if let Ok(engine) = std::env::var("SIGHTGLASS_TEST_ENGINE") {
        // Use the engine specified by the environment variable. We use this to cache built
        // `libwasmtime_bench_api.so`s in CI.
        engine.into()
    } else {
        // Make sure we only ever build Wasmtime once, and don't have N threads build it in parallel
        // and race to be the one to save it onto the file system.
        static BUILD_WASMTIME: std::sync::Once = std::sync::Once::new();
        BUILD_WASMTIME.call_once(|| {
            if sightglass_build::path::get_known_engine_path("wasmtime")
                .unwrap()
                .is_file()
            {
                // A wasmtime engine is already built!
                return;
            }

            // Use this instead of `eprintln!` to avoid `cargo test`'s stdio
            // capturing.
            use std::io::Write;
            drop(writeln!(
                std::io::stderr(),
                "**************************************************************\n\
                 *** Building Wasmtime engine; this may take a few minutes. ***\n\
                 **************************************************************"
            ));

            let status = Command::cargo_bin("sightglass-cli")
                .unwrap()
                .current_dir("../..") // Run in the root of the repo.
                .arg("build-engine")
                .arg("wasmtime")
                .status()
                .expect("failed to run `sightglass-cli build-engine`");
            assert!(status.success());
        });
        sightglass_build::path::get_known_engine_path("wasmtime").unwrap()
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
    format!("../../benchmarks-next/{}/benchmark.wasm", benchmark_name).into()
}
