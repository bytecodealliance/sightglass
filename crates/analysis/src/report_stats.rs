use crate::summarize::{coefficient_of_variation, mean, percentile_from_sorted, std_deviation};
use sightglass_data::{extract_benchmark_name, Measurement, Phase};
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur during benchmark analysis
#[derive(Debug, Error)]
pub enum AnalysisError {
    #[error("No measurements available for analysis")]
    InsufficientData,
    #[error("Insufficient sample size: need at least {required} samples, got {actual}")]
    InsufficientSampleSize { required: usize, actual: usize },
    #[error("No measurements found for event '{event}' in phase '{phase:?}'")]
    NoMatchingMeasurements { phase: Phase, event: String },
    #[error("Invalid significance level: {level} (must be between 0 and 1)")]
    InvalidSignificanceLevel { level: f64 },
    #[error("Statistical analysis failed: {0}")]
    StatisticalError(String),
}

/// Configuration for benchmark report generation
#[derive(Debug, Clone)]
pub struct ReportConfig {
    pub primary_event: String,
    pub target_phase: Phase,
    pub significance_level: f64,
    pub baseline_engine: String,
}

impl ReportConfig {
    pub fn new(baseline_engine: impl Into<String>) -> Self {
        Self {
            primary_event: "cycles".to_string(),
            target_phase: Phase::Execution,
            significance_level: 0.05,
            baseline_engine: baseline_engine.into(),
        }
    }

    pub fn with_event(mut self, event: impl Into<String>) -> Self {
        self.primary_event = event.into();
        self
    }

    pub fn with_phase(mut self, phase: Phase) -> Self {
        self.target_phase = phase;
        self
    }

    pub fn with_significance_level(mut self, level: f64) -> Self {
        self.significance_level = level;
        self
    }

    pub fn with_baseline_engine(mut self, engine: impl Into<String>) -> Self {
        self.baseline_engine = engine.into();
        self
    }
}

/// Statistics relative to a baseline measurement
#[derive(Debug, Clone, serde::Serialize)]
pub struct RelativeStats {
    pub p25_delta_pct: f64,
    pub mean_delta_pct: f64,
    pub is_significant: bool,
    pub effect_size_mean_diff: f64,
    pub speedup_ratio: f64,
    pub confidence_interval_half_width: Option<f64>,
    pub speedup_confidence_interval: Option<f64>,
}

/// Statistics calculated for a benchmark grouped by prefix.
#[derive(Debug, Clone, serde::Serialize)]
pub struct BenchmarkStats {
    // Core statistics (always present)
    pub cv: f64,
    pub std: f64,
    pub mean: f64,
    pub median: f64, // p50 - consolidated into median
    pub p25: f64,
    pub p75: f64,
    pub min: f64,
    pub max: f64,
    pub significance_level: f64,

    // Relative statistics (only when comparing to baseline)
    pub relative_stats: Option<RelativeStats>,
}

/// Data structure representing aggregated statistics for a benchmark.
#[derive(Debug)]
pub struct BenchmarkData {
    pub name: String,
    pub stats_by_engine: HashMap<String, BenchmarkStats>,
    pub baseline_stats: BenchmarkStats,
}

/// Calculate statistics for all benchmarks grouped by engine.
pub fn calculate_benchmark_stats<'a>(
    measurements: &[Measurement<'a>],
    config: &ReportConfig,
) -> Result<HashMap<String, HashMap<String, BenchmarkStats>>, AnalysisError> {
    // Early validation
    if measurements.is_empty() {
        return Err(AnalysisError::InsufficientData);
    }

    let mut results: HashMap<String, HashMap<String, BenchmarkStats>> = HashMap::new();

    // Group measurements by benchmark and engine
    let mut grouped: HashMap<String, HashMap<String, Vec<u64>>> = HashMap::new();

    for measurement in measurements {
        // Only process measurements matching the configured phase and event
        if measurement.phase != config.target_phase || measurement.event != config.primary_event {
            continue;
        }

        let benchmark = extract_benchmark_name(&measurement.wasm);
        let engine_name = measurement.engine.to_string();

        grouped
            .entry(benchmark)
            .or_default()
            .entry(engine_name)
            .or_default()
            .push(measurement.count);
    }

    // Check if we found any matching measurements
    if grouped.is_empty() {
        return Err(AnalysisError::NoMatchingMeasurements {
            phase: config.target_phase,
            event: config.primary_event.clone(),
        });
    }

    // Calculate statistics for each group
    for (benchmark, benchmark_measurements_by_engine) in grouped {
        let mut benchmark_results = HashMap::new();

        // Use the baseline engine from config
        let baseline_engine = config.baseline_engine.as_str();

        // Ensure baseline engine exists in measurements
        let baseline_measurements = benchmark_measurements_by_engine
            .get(baseline_engine)
            .ok_or_else(|| {
                AnalysisError::StatisticalError(format!(
                    "Baseline engine '{baseline_engine}' not found in measurements for benchmark '{benchmark}'"
                ))
            })?;

        let baseline_measurements = Some(baseline_measurements.as_slice());
        let significance_level = config.significance_level;

        for (engine, counts) in benchmark_measurements_by_engine.iter() {
            let stats = if engine == baseline_engine {
                calculate_stats_for_measurements(counts, None, significance_level)?
            } else {
                calculate_stats_for_measurements(counts, baseline_measurements, significance_level)?
            };
            benchmark_results.insert(engine.clone(), stats);
        }

        results.insert(benchmark, benchmark_results);
    }

    Ok(results)
}

/// Calculate statistics for a group of measurements.
fn calculate_stats_for_measurements(
    measurements: &[u64],
    baseline_measurements: Option<&[u64]>,
    significance_level: f64,
) -> Result<BenchmarkStats, AnalysisError> {
    // Validate minimum sample size for measurements
    if measurements.len() < 3 {
        return Err(AnalysisError::InsufficientSampleSize {
            required: 3,
            actual: measurements.len(),
        });
    }

    // Validate baseline sample size if provided
    if let Some(baseline) = baseline_measurements {
        if baseline.len() < 3 {
            return Err(AnalysisError::InsufficientSampleSize {
                required: 3,
                actual: baseline.len(),
            });
        }
    }

    // Validate significance level
    if !(0.0..=1.0).contains(&significance_level) {
        return Err(AnalysisError::InvalidSignificanceLevel {
            level: significance_level,
        });
    }

    let mean = mean(measurements);
    let std = std_deviation(measurements);
    let mut sorted_measurements = measurements.to_vec();
    sorted_measurements.sort();
    let p25 = percentile_from_sorted(&sorted_measurements, 25.0);
    let median = percentile_from_sorted(&sorted_measurements, 50.0);
    let p75 = percentile_from_sorted(&sorted_measurements, 75.0);
    let min = sorted_measurements[0] as f64;
    let max = sorted_measurements[sorted_measurements.len() - 1] as f64;
    let cv = coefficient_of_variation(measurements);

    // Calculate relative statistics if we have baseline data
    let relative_stats = if let Some(baseline_measurements) = baseline_measurements {
        let baseline_mean = crate::summarize::mean(baseline_measurements);
        let mut baseline_sorted = baseline_measurements.to_vec();
        baseline_sorted.sort();
        let baseline_p25 = percentile_from_sorted(&baseline_sorted, 25.0);
        let p25_delta_pct = (p25 - baseline_p25) / (p25 + baseline_p25) * 100.0;
        let mean_delta_pct = (mean - baseline_mean) / (mean + baseline_mean) * 100.0;

        // Always calculate basic relative metrics
        let effect_size_mean_diff = mean - baseline_mean;
        let speedup_ratio = if baseline_mean > 0.0 {
            mean / baseline_mean
        } else {
            1.0
        };

        // Use behrens-fisher for statistical significance
        let current_stats: behrens_fisher::Stats = measurements.iter().map(|&c| c as f64).collect();
        let baseline_stats: behrens_fisher::Stats =
            baseline_measurements.iter().map(|&c| c as f64).collect();

        if let Ok(ci) = behrens_fisher::confidence_interval(
            1.0 - significance_level,
            current_stats,
            baseline_stats,
        ) {
            let is_significant = effect_size_mean_diff.abs() > ci.abs();
            let speedup_confidence_interval = if baseline_mean > 0.0 {
                ci / baseline_mean
            } else {
                0.0
            };

            Some(RelativeStats {
                p25_delta_pct,
                mean_delta_pct,
                is_significant,
                effect_size_mean_diff,
                speedup_ratio,
                confidence_interval_half_width: Some(ci),
                speedup_confidence_interval: Some(speedup_confidence_interval),
            })
        } else {
            Some(RelativeStats {
                p25_delta_pct,
                mean_delta_pct,
                is_significant: false,
                effect_size_mean_diff,
                speedup_ratio,
                confidence_interval_half_width: None,
                speedup_confidence_interval: None,
            })
        }
    } else {
        None
    };

    Ok(BenchmarkStats {
        cv,
        std,
        mean,
        median,
        p25,
        p75,
        min,
        max,
        significance_level,
        relative_stats,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_stats_for_measurements() {
        let measurements = vec![1, 2, 3, 4, 5];
        let stats = calculate_stats_for_measurements(&measurements, None, 0.05)
            .expect("Should calculate stats successfully");

        assert_eq!(stats.mean, 3.0);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
        assert_eq!(stats.median, 3.0); // changed from p50 to median
        assert!(stats.cv > 0.0);
        assert!(stats.relative_stats.is_none()); // No baseline provided
    }

    #[test]
    fn test_calculate_stats_with_baseline() {
        let measurements = vec![5, 6, 7, 8, 9];
        let baseline = vec![1, 2, 3, 4, 5];
        let stats = calculate_stats_for_measurements(&measurements, Some(&baseline), 0.05)
            .expect("Should calculate stats successfully");

        assert_eq!(stats.mean, 7.0);
        assert!(stats.relative_stats.is_some());

        let rel = stats.relative_stats.unwrap();
        assert!(rel.mean_delta_pct > 0.0); // Should be positive (measurements higher than baseline)
        assert!(rel.speedup_ratio > 1.0); // Should be > 1 since measurements are higher than baseline
        assert!(rel.effect_size_mean_diff > 0.0); // Should be positive mean difference
    }

    #[test]
    fn test_insufficient_sample_size() {
        let measurements = vec![1, 2];
        let result = calculate_stats_for_measurements(&measurements, None, 0.05);
        assert!(matches!(
            result,
            Err(AnalysisError::InsufficientSampleSize {
                required: 3,
                actual: 2
            })
        ));
    }

    #[test]
    fn test_invalid_significance_level() {
        let measurements = vec![1, 2, 3, 4, 5];
        let result = calculate_stats_for_measurements(&measurements, None, 1.5);
        assert!(
            matches!(result, Err(AnalysisError::InvalidSignificanceLevel { level }) if level == 1.5)
        );
    }
}
