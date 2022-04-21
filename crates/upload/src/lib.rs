//! Upload Sightglass data into an ElasticSearch database.
mod database;
mod measurement;

use crate::database::Database;
use crate::measurement::UploadMeasurement;
use anyhow::Result;
use sightglass_data::Measurement;
use sightglass_fingerprint::{Benchmark, Engine, Machine};
use std::collections::{HashMap, HashSet};

/// Upload `measurements` to the `server`. This will replace several fields of the raw [Measurement]
/// with fingerprinted data from the current server; this adds useful metadata to the results.
pub fn upload(server: &str, dry_run: bool, measurements: &Vec<Measurement>) -> Result<()> {
    let database = Database::new(server.to_string(), dry_run);

    // De-duplicate all of the engines and benchmarks used in this result set.
    let mut found_engines = HashSet::new();
    let mut found_benchmarks = HashSet::new();
    for m in measurements {
        found_engines.insert(m.engine.clone());
        found_benchmarks.insert(m.wasm.clone());
    }

    // Insert each fingerprinted version of an engine.
    let mut engines = HashMap::new();
    for engine_path in found_engines.into_iter() {
        let engine = Engine::fingerprint(engine_path.as_ref())?;
        let engine_id = database.create_if_not_exists("engines", &engine, &engine.name)?;
        log::debug!("Mapping engine: {} -> {}", &engine_path, &engine_id);
        engines.insert(engine_path, engine_id);
    }

    // Insert each fingerprinted version of a benchmark;
    let mut benchmarks = HashMap::new();
    for benchmark_path in found_benchmarks.into_iter() {
        let benchmark = Benchmark::fingerprint(benchmark_path.as_ref())?;
        let benchmark_id =
            database.create_if_not_exists("benchmarks", &benchmark, &benchmark.name)?;
        log::debug!(
            "Mapping benchmark: {} -> {}",
            &benchmark_path,
            &benchmark_id
        );
        benchmarks.insert(benchmark_path, benchmark_id);
    }

    // Fingerprint the current machine.
    let machine = Machine::fingerprint()?;
    let machine = database.create_if_not_exists("machines", &machine, &machine.name)?;

    // Upload the measurements.
    for m in measurements {
        let engine = engines.get(m.engine.as_ref()).unwrap().as_ref();
        let benchmark = benchmarks.get(m.wasm.as_ref()).unwrap().as_ref();
        let upload = UploadMeasurement::convert(&machine, engine, benchmark, m);
        database.create("measurements", &upload, None)?;
    }

    Ok(())
}
