use anyhow::Result;
use sightglass_artifact::EngineBuilder;
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
    location: EngineBuilder,
}

impl BuildEngineCommand {
    pub fn execute(&self) -> Result<()> {
        let engine = self.location.as_engine();
        if !engine.exists() || self.force_rebuild {
            self.location.build()?;
        }
        println!("{}", engine.path().display());
        Ok(())
    }
}
