use anyhow::Result;
use sightglass_artifact::{build_engine, get_known_engine_path};
use structopt::StructOpt;

/// Build a Wasm benchmark from either an engine-ref or a Dockerfile and print the path the
/// generated library.
#[derive(Debug, StructOpt)]
#[structopt(name = "build-engine")]
pub struct BuildEngineCommand {
    /// Force this tool to rebuild the benchmark, if possible.
    #[structopt(long, short)]
    force_rebuild: bool,

    /// Either a well-known engine (e.g. `wasmtime` or `wasmtime@92350bf2` or
    /// `wasmtime@92350bf2@https://github.com/user/wasmtime`) or a path to a Dockerfile.
    #[structopt(index = 1, required = true, value_name = "ENGINE-REF OR DOCKERFILE")]
    location: String,
}

impl BuildEngineCommand {
    pub fn execute(&self) -> Result<()> {
        let engine_path = get_known_engine_path(&self.location)?;
        if !engine_path.exists() || self.force_rebuild {
            build_engine(&self.location, &engine_path)?;
        }
        println!("{}", engine_path.display());
        Ok(())
    }
}
