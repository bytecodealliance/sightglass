use super::util::sightglass_cli;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use scraper::{Html, Selector};
use std::fs;
use tempfile::TempDir;

fn test_results_json() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/results.json")
}

fn multi_engine_v38_json() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/multi_engine_v38.json")
}

fn multi_engine_v38_epoch_json() -> &'static str {
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/multi_engine_v38_epoch.json"
    )
}

fn count_css_elements(html_content: &str, selector: &str) -> usize {
    let document = Html::parse_document(html_content);
    let css_selector = Selector::parse(selector).unwrap();
    document.select(&css_selector).count()
}

#[test]
fn report_generates_valid_html() {
    // Test the happy path: report generation with multi-engine data
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("report.html");

    sightglass_cli()
        .arg("report")
        .arg("--output-file")
        .arg(&output_path)
        .arg(multi_engine_v38_json())
        .arg(multi_engine_v38_epoch_json())
        .assert()
        .success();

    let html_content = fs::read_to_string(&output_path).unwrap();

    // Verify essential report elements exist
    assert!(html_content.contains("<html"), "Should be valid HTML");
    assert!(html_content.contains("📊"), "Should mark baseline engine");
    assert!(
        html_content.contains("% slower") || html_content.contains("% faster"),
        "Should show performance comparisons"
    );
    assert!(
        count_css_elements(&html_content, "table") > 0,
        "Should have tables"
    );

    // Verify charts exist
    assert!(
        html_content.contains("\"title\":\"bz2\""),
        "Should have benchmark charts"
    );

    // Verify statistical indicators
    assert!(
        html_content.contains("CV") && html_content.contains("%"),
        "Should show coefficient of variation"
    );
}

#[test]
fn report_command_line_options() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("report.html");

    // Test various command-line options work
    sightglass_cli()
        .arg("report")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Generate an HTML report"));

    // Test custom baseline
    sightglass_cli()
        .arg("report")
        .arg("--baseline-engine")
        .arg("engines/wasmtime/wasmtime-v38/libengine.dylib (-W epoch-interruption=y)")
        .arg("--output-file")
        .arg(&output_path)
        .arg(multi_engine_v38_json())
        .arg(multi_engine_v38_epoch_json())
        .assert()
        .success();

    let html = fs::read_to_string(&output_path).unwrap();
    assert!(
        html.contains("(-W epoch-interruption=y)"),
        "Should use custom baseline"
    );

    // Test custom significance level
    let output_path2 = temp_dir.path().join("report2.html");
    sightglass_cli()
        .arg("report")
        .arg("--significance-level")
        .arg("0.01")
        .arg("--output-file")
        .arg(&output_path2)
        .arg(test_results_json())
        .assert()
        .success();

    let html2 = fs::read_to_string(&output_path2).unwrap();
    assert!(
        html2.contains("99") && html2.contains("%"),
        "Should show 99% confidence"
    );

    // Test custom event and phase
    let output_path3 = temp_dir.path().join("report3.html");
    sightglass_cli()
        .arg("report")
        .arg("--event")
        .arg("nanoseconds")
        .arg("--phase")
        .arg("compilation")
        .arg("--output-file")
        .arg(&output_path3)
        .arg(test_results_json())
        .assert()
        .success();
}

#[test]
fn report_error_cases() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("report.html");

    // Missing input files
    sightglass_cli()
        .arg("report")
        .arg("--output-file")
        .arg(&output_path)
        .assert()
        .failure();

    // Nonexistent input file
    sightglass_cli()
        .arg("report")
        .arg("--output-file")
        .arg(&output_path)
        .arg("nonexistent.json")
        .assert()
        .failure();

    // Invalid significance level
    sightglass_cli()
        .arg("report")
        .arg("--significance-level")
        .arg("1.5")
        .arg("--output-file")
        .arg(&output_path)
        .arg(test_results_json())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid significance level"));

    // Invalid event
    sightglass_cli()
        .arg("report")
        .arg("--event")
        .arg("nonexistent_event")
        .arg("--output-file")
        .arg(&output_path)
        .arg(test_results_json())
        .assert()
        .failure()
        .stderr(predicate::str::contains("No measurements found"));
}

#[test]
fn report_statistical_correctness() {
    // Verify that changing significance level affects results
    let temp_dir = TempDir::new().unwrap();
    let input_files = &[multi_engine_v38_json(), multi_engine_v38_epoch_json()];

    // Strict significance
    let strict_output = temp_dir.path().join("strict.html");
    sightglass_cli()
        .arg("report")
        .arg("--significance-level")
        .arg("0.01")
        .arg("--output-file")
        .arg(&strict_output)
        .arg(input_files[0])
        .arg(input_files[1])
        .assert()
        .success();

    // Lenient significance
    let lenient_output = temp_dir.path().join("lenient.html");
    sightglass_cli()
        .arg("report")
        .arg("--significance-level")
        .arg("0.10")
        .arg("--output-file")
        .arg(&lenient_output)
        .arg(input_files[0])
        .arg(input_files[1])
        .assert()
        .success();

    let strict_html = fs::read_to_string(&strict_output).unwrap();
    let lenient_html = fs::read_to_string(&lenient_output).unwrap();

    // Lenient should find same or more significant results
    let strict_significant =
        count_css_elements(&strict_html, ".slower") + count_css_elements(&strict_html, ".faster");
    let lenient_significant =
        count_css_elements(&lenient_html, ".slower") + count_css_elements(&lenient_html, ".faster");

    assert!(
        lenient_significant >= strict_significant,
        "Lenient significance should find more results"
    );
}
