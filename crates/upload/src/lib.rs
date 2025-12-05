//! Upload Sightglass data into an ElasticSearch database.
mod database;
mod measurement;

use crate::database::Database;
use crate::measurement::UploadMeasurement;
use anyhow::{Context, Result};
pub use measurement::MeasurementPackage;
use sightglass_data::Measurement;
use sightglass_fingerprint::{Benchmark, Engine, Machine};
use std::collections::{HashMap, HashSet};
use std::io;

/// Upload `measurements` to the `server`.
///
/// Uploading to a database will replace several fields of the raw [Measurement]
/// with fingerprinted data from the current server; this adds useful metadata
/// to the results. E.g., the `arch` field in [Measurement] is expanded to the
/// more-complete [Machine] fingerprint.
pub fn upload(
    server: &str,
    batch_size: usize,
    dry_run: bool,
    measurements: Vec<Measurement>,
) -> Result<()> {
    let package = package(measurements)?;

    if dry_run {
        serde_json::to_writer(io::stdout(), &package)
            .context("failed to write measurement package to stdout")?;
    }

    upload_package(server, batch_size, dry_run, package)
}

/// Package up the [Measurement]s alongside the fingerprint data from the
/// current system.
///
/// The concept of "packaging up" the data allows for saving the entire
/// data collection for later upload in environments where access to the
/// database is impossible.
pub fn package(measurements: Vec<Measurement>) -> Result<MeasurementPackage> {
    // De-duplicate all of the engines and benchmarks used in this result set.
    let mut found_engines = HashSet::new();
    let mut found_benchmarks = HashSet::new();
    for m in measurements.iter() {
        found_engines.insert(m.engine.name.clone());
        found_benchmarks.insert(m.wasm.clone());
    }

    // Map each engine path to its fingerprinted data.
    let mut engines = HashMap::new();
    for engine_path in found_engines.into_iter() {
        let engine = Engine::fingerprint(engine_path.as_ref())?;
        engines.insert(engine_path, engine);
    }

    // Map each benchmark path to its fingerprinted data.
    let mut benchmarks = HashMap::new();
    for benchmark_path in found_benchmarks.into_iter() {
        let benchmark = Benchmark::fingerprint(benchmark_path.as_ref())?;
        benchmarks.insert(benchmark_path, benchmark);
    }

    // Fingerprint the current machine.
    let machine = Machine::fingerprint()?;

    // Capture the current time.
    let datetime = chrono::Local::now().to_rfc3339().into();

    Ok(MeasurementPackage {
        measurements,
        engines,
        benchmarks,
        machine,
        datetime,
    })
}

/// Upload the packaged measurements and fingerprint data to the database.
///
/// This adds records to the database for each kind of record in the package
/// (measurements, machine, engines, benchmarks), avoiding new records for
/// fingerprinted data that is already present in the database (e.g., we do not
/// want multiple records for the same machine). If `dry_run` is set no records
/// are actually inserted, only logged.
///
/// As described in the [tuning documentation], the optimal ElasticSearch
/// `batch_size` must be determined through experimentation. In a small
/// experiment, increasing the batch size saw diminishing returns after
/// `1000-2000`.
///
/// [tuning documentation]:
///     https://www.elastic.co/guide/en/elasticsearch/reference/master/tune-for-indexing-speed.html#_use_bulk_requests
pub fn upload_package(
    server: &str,
    batch_size: usize,
    dry_run: bool,
    package: MeasurementPackage,
) -> Result<()> {
    let database = Database::new(server.to_string(), dry_run);

    // Insert each fingerprinted version of an engine.
    let mut engines = HashMap::new();
    for (path, fingerprint) in package.engines.into_iter() {
        let id = database.create_if_not_exists("engines", &fingerprint, &fingerprint.id)?;
        log::debug!("Mapping engine: {} -> {}", &path, &id);
        engines.insert(path, id);
    }

    // Insert each fingerprinted version of a benchmark.
    let mut benchmarks = HashMap::new();
    for (path, fingerprint) in package.benchmarks.into_iter() {
        let id = database.create_if_not_exists("benchmarks", &fingerprint, &fingerprint.id)?;
        log::debug!("Mapping benchmark: {} -> {}", &path, &id);
        benchmarks.insert(path, id);
    }

    // Insert the machine fingerprint.
    let machine =
        database.create_if_not_exists("machines", &package.machine, &package.machine.id)?;

    // Insert all of the measurements.
    for batch in package.measurements.chunks(batch_size) {
        let batch = batch
            .iter()
            .map(|m| {
                UploadMeasurement::map_and_convert(
                    &machine,
                    &engines,
                    &benchmarks,
                    &package.datetime,
                    m,
                )
            })
            .collect::<Vec<_>>();
        database.create_batched("measurements", &batch)?;
    }

    Ok(())
}
