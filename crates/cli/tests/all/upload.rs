//! Test `sightglass-cli upload`.

// Because the `results.json` contains `*.so` suffixes for the engine, this test
// can only run where the fingerprinted engine will have a matching suffix,
// i.e., Linux.
#[cfg(target_os = "linux")]
#[test]
fn upload_dryrun() {
    let assert = assert_cmd::assert::OutputAssertExt::assert(
        crate::util::sightglass_cli()
            .arg("upload-elastic")
            .arg("--dry-run")
            .arg("--input-file")
            .arg("tests/results.json")
            .arg("--batch-size")
            .arg("200")
            .env("RUST_LOG", "debug"),
    );

    // Gather up the logged output from stderr.
    let stderr = std::str::from_utf8(&assert.get_output().stderr).unwrap();
    eprintln!("=== stderr ===\n{stderr}\n===========");

    // Gather the fingerprints of the system under test.
    let engine = sightglass_fingerprint::Engine::fingerprint(crate::util::test_engine()).unwrap();
    let benchmark =
        sightglass_fingerprint::Benchmark::fingerprint(crate::util::benchmark("noop")).unwrap();
    let machine = sightglass_fingerprint::Machine::fingerprint().unwrap();

    // Check that we upload measurement records for each of the measurements in the file.
    let num_uploaded_batches = stderr
        .matches("Batching up 200 records to index 'measurements'")
        .count();
    assert_eq!(num_uploaded_batches, 3);

    // Also, heck that we create records for the engine/machine/benchmark.
    use predicates::prelude::*;
    use predicates::str::*;

    assert
        .stderr(
            contains(format!(
                r#"Creating record in 'engines' with ID Some("{}")"#,
                engine.id
            ))
            .and(contains(format!(
                r#"Creating record in 'machines' with ID Some("{}")"#,
                machine.id
            )))
            .and(contains(format!(
                r#"Creating record in 'benchmarks' with ID Some("{}")"#,
                benchmark.id
            ))),
        )
        .success();
}
