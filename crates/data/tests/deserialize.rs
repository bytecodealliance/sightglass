use sightglass_data::{Format, Measurement};
use std::fs::File;

#[test]
fn json() {
    let file = File::open("tests/results.json").unwrap();
    let measurements: Vec<Measurement> = Format::Json.read(file).unwrap();
    assert_eq!(measurements.len(), 9);
}

#[test]
fn csv_no_headers() {
    let file = File::open("tests/results-no-headers.csv").unwrap();
    let format = Format::csv(false);
    let measurements: Vec<Measurement> = format.read(file).unwrap();
    assert_eq!(measurements.len(), 9);
}

#[test]
fn csv_with_headers() {
    let file = File::open("tests/results-with-headers.csv").unwrap();
    let format = Format::csv(true);
    let measurements: Vec<Measurement> = format.read(file).unwrap();
    assert_eq!(measurements.len(), 9);
}
