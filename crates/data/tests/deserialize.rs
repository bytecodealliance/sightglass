use sightglass_data::{Format, Measurement};
use std::fs::File;

#[test]
fn json() {
    let file = File::open("tests/results.json").unwrap();
    let measurements: Vec<Measurement> = Format::Json.read(file).unwrap();
    assert_eq!(measurements.len(), 9);
}

#[test]
fn csv() {
    let file = File::open("tests/results.csv").unwrap();
    let measurements: Vec<Measurement> = Format::Csv.read(file).unwrap();
    assert_eq!(measurements.len(), 9);
}
