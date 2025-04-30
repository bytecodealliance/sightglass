use std::{
    cell::Cell,
    collections::HashMap,
    fs::File,
    hash::{Hash, Hasher},
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

use serde::Serialize;
use sightglass_analysis::report_stats::{calculate_benchmark_stats, BenchmarkStats, ReportConfig};
use sightglass_data::{extract_benchmark_name, Engine, Format, Measurement, Phase};
use structopt::StructOpt;
use vega_lite_4::{
    AxisBuilder, ColorClassBuilder, EdEncodingBuilder, LegendBuilder, Mark, NormalizedSpecBuilder,
    XClassBuilder, YClassBuilder,
};

const TEMPLATE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/report.jinja"));

/// Generate an HTML report for a given set of raw inputs
#[derive(Debug, StructOpt)]
#[structopt(name = "report")]
pub struct ReportCommand {
    /// The format of the input data. Either 'json' or 'csv'; if not provided
    /// then we will attempt to infer it from provided filenames, falling back to json.
    #[structopt(short = "i", long = "input-format")]
    input_format: Option<Format>,

    /// Output HTML file path
    #[structopt(short = "o", long = "output-file", default_value = "report.html")]
    output_path: PathBuf,

    /// Name of the baseline to use; if not provided, the first engine encountered
    /// in the ordered input files will be used.
    #[structopt(short = "b", long = "baseline-engine")]
    baseline_engine: Option<String>,

    /// Significance level for statistical tests (default: 0.05 for 95% confidence)
    #[structopt(long = "significance-level", default_value = "0.05")]
    significance_level: f64,

    /// Primary event to analyze (default: cycles)
    #[structopt(long = "event", default_value = "cycles")]
    primary_event: String,

    /// Target phase to analyze (default: execution)
    #[structopt(long = "phase", default_value = "execution")]
    target_phase: Phase,

    /// Path to the file(s) that will be read from, or none to indicate stdin (default).
    #[structopt(min_values = 1)]
    input_files: Vec<PathBuf>,
}

#[derive(Debug, Serialize)]
struct Chart {
    json: String,
    id: u64,
}

#[derive(Debug, Serialize)]
struct BenchmarkData {
    name: String,
    chart: Chart,
    stats_by_engine: HashMap<String, BenchmarkStats>,
    baseline_stats: BenchmarkStats,
    short_names: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
struct SightglassStats {
    baseline_engine: String,
    benchmarks: Vec<BenchmarkData>,
}

fn get_available_events(measurements: &[Measurement]) -> String {
    let mut events: Vec<&str> = measurements
        .iter()
        .map(|m| m.event.as_ref())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    events.sort();
    events.join(", ")
}

fn get_available_phases(measurements: &[Measurement]) -> String {
    let mut phases: Vec<String> = measurements
        .iter()
        .map(|m| format!("{:?}", m.phase))
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    phases.sort();
    phases.join(", ")
}

fn determine_baseline_engine(
    measurements: &[Measurement],
    requested_baseline: Option<&String>,
) -> String {
    if let Some(baseline) = requested_baseline {
        return baseline.clone();
    }

    // Get first available engine from measurements
    measurements
        .iter()
        .map(|m| m.engine.to_string())
        .next()
        .unwrap_or_else(|| "baseline".to_string())
}

fn parse_input(
    format: Option<Format>,
    path: impl AsRef<Path>,
) -> anyhow::Result<Vec<Measurement<'static>>> {
    let format = format
        .or_else(|| match path.as_ref().extension()?.to_str()? {
            "json" => Some(Format::Json),
            "csv" => Some(Format::Csv {
                headers: Cell::new(true),
            }),
            _ => None,
        })
        .unwrap_or(Format::Json);

    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let measurements: Vec<Measurement> = format.read(reader)?;

    Ok(measurements)
}

impl ReportCommand {
    fn plot_benchmark(
        &self,
        bstats: &BenchmarkStats,
        benchmark: &str,
        measurements: &[&Measurement],
    ) -> anyhow::Result<String> {
        use vega_lite_4::{self as vl, VegaliteBuilder};

        // Create a simple data structure for vega-lite
        #[derive(Debug, serde::Serialize)]
        struct ChartDataPoint {
            count: u64,
            engine: String,
            p25_delta_pct: f64,
        }

        let chart_data: Vec<ChartDataPoint> = measurements
            .iter()
            .map(|m| ChartDataPoint {
                count: m.count,
                engine: m.engine.to_string(),
                p25_delta_pct: (100.0 * (m.count as f64 - bstats.p25)
                    / ((m.count as f64 + bstats.p25) / 2.0)),
            })
            .collect();
        let cycles_chart = NormalizedSpecBuilder::default()
            .data(&chart_data)
            .mark(Mark::Boxplot)
            .encoding(
                EdEncodingBuilder::default()
                    .x(XClassBuilder::default()
                        .field("count")
                        .position_def_type(vl::Type::Quantitative)
                        .axis(AxisBuilder::default().title("cycles").build()?)
                        .build()?)
                    .y(YClassBuilder::default()
                        .field("engine")
                        .position_def_type(vl::Type::Nominal)
                        .axis(AxisBuilder::default().labels(false).build()?)
                        .build()?)
                    .color(
                        ColorClassBuilder::default()
                            .field("engine")
                            .legend(
                                LegendBuilder::default()
                                    .title("Engine")
                                    .orient(vega_lite_4::LegendOrient::Bottom)
                                    .label_limit(600.0)
                                    .direction(vega_lite_4::Orientation::Vertical)
                                    .columns(1.0)
                                    .build()?,
                            )
                            .build()?,
                    )
                    .build()?,
            )
            .build()?;
        let pct_chart = NormalizedSpecBuilder::default()
            .data(&chart_data)
            .mark(Mark::Boxplot)
            .encoding(
                EdEncodingBuilder::default()
                    .x(XClassBuilder::default()
                        .field("p25_delta_pct")
                        .position_def_type(vl::Type::Quantitative)
                        .axis(
                            AxisBuilder::default()
                                .title("delta p25 as percentage")
                                .build()?,
                        )
                        .build()?)
                    .y(YClassBuilder::default()
                        .field("engine")
                        .position_def_type(vl::Type::Nominal)
                        .axis(AxisBuilder::default().labels(false).build()?)
                        .build()?)
                    .color(
                        ColorClassBuilder::default()
                            .field("engine")
                            .legend(
                                LegendBuilder::default()
                                    .title("Engine")
                                    .orient(vega_lite_4::LegendOrient::Bottom)
                                    .label_limit(600.0)
                                    .direction(vega_lite_4::Orientation::Vertical)
                                    .columns(1.0)
                                    .build()?,
                            )
                            .build()?,
                    )
                    .build()?,
            )
            .build()?;

        let chart = VegaliteBuilder::default()
            .title(benchmark)
            .hconcat(vec![cycles_chart, pct_chart])
            .build()?;

        Ok(chart.to_string()?)
    }

    fn compute_stats(&self, measurements: &[Measurement]) -> anyhow::Result<SightglassStats> {
        // Determine baseline engine from CLI args or first available
        let baseline_engine =
            determine_baseline_engine(measurements, self.baseline_engine.as_ref());

        // Create ReportConfig from CLI arguments (still using original names for logic)
        let config = ReportConfig::new(baseline_engine)
            .with_event(&self.primary_event)
            .with_phase(self.target_phase)
            .with_significance_level(self.significance_level);

        // Use the new calculate_benchmark_stats function with ReportConfig
        let benchmark_stats = calculate_benchmark_stats(measurements, &config)?;

        // Check if we found any matching data
        if benchmark_stats.is_empty() {
            anyhow::bail!(
                "No measurements found matching the specified criteria:\n\
                 - Event: {}\n\
                 - Phase: {:?}\n\
                 \n\
                 Available events: {}\n\
                 Available phases: {}",
                config.primary_event,
                config.target_phase,
                get_available_events(measurements),
                get_available_phases(measurements)
            );
        }

        let mut benchmarks_data: Vec<BenchmarkData> = Vec::new();

        // Keep original baseline name for display
        let baseline_engine_for_display = config.baseline_engine.clone();

        // Convert the calculated benchmark stats to our display format
        for (benchmark_name, stats_by_engine) in benchmark_stats {
            // Use baseline engine from config
            let baseline_engine = config.baseline_engine.as_str();

            let baseline_stats = stats_by_engine
                .get(baseline_engine)
                .cloned()
                .unwrap_or_else(|| stats_by_engine.values().next().unwrap().clone());

            // Get measurements for this benchmark that match our config filters
            let benchmark_measurements: Vec<&Measurement> = measurements
                .iter()
                .filter(|m| {
                    extract_benchmark_name(&m.wasm) == benchmark_name
                        && m.phase == config.target_phase
                        && m.event == config.primary_event
                })
                .collect();

            let chart_json =
                self.plot_benchmark(&baseline_stats, &benchmark_name, &benchmark_measurements)?;
            let id = {
                let mut h = std::hash::DefaultHasher::new();
                benchmark_name.hash(&mut h);
                h.finish()
            };

            // Compute short names for this benchmark's engines
            // Get unique engines from the measurements for this benchmark
            let benchmark_engines: Vec<&Engine> = benchmark_measurements
                .iter()
                .map(|m| &m.engine)
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();

            // Find the baseline engine
            let baseline_engine_obj = benchmark_engines
                .iter()
                .find(|e| e.to_string() == baseline_engine)
                .or_else(|| benchmark_engines.first())
                .unwrap();

            // Compute shortened names relative to baseline
            let short_names: HashMap<String, String> = benchmark_engines
                .iter()
                .map(|engine| {
                    let full = engine.to_string();
                    if *engine == *baseline_engine_obj {
                        (full.clone(), full)
                    } else {
                        let (_, short) = baseline_engine_obj.relative_labels(engine);
                        // Add ellipsis if prefix was removed (short will be a suffix of full)
                        let short_with_ellipsis = if short != full && full.ends_with(&short) {
                            format!("…{}", short)
                        } else {
                            short
                        };
                        (full, short_with_ellipsis)
                    }
                })
                .collect();

            benchmarks_data.push(BenchmarkData {
                name: benchmark_name,
                chart: Chart {
                    json: chart_json,
                    id,
                },
                stats_by_engine,
                baseline_stats,
                short_names,
            });
        }

        // Sort benchmarks by name for consistent ordering
        benchmarks_data.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(SightglassStats {
            baseline_engine: baseline_engine_for_display,
            benchmarks: benchmarks_data,
        })
    }

    pub fn execute(&self) -> anyhow::Result<()> {
        let mut all_measurements = Vec::new();

        for input_file in &self.input_files {
            let measurements = parse_input(self.input_format.clone(), input_file)?;
            all_measurements.extend(measurements);
        }

        let stats = self.compute_stats(&all_measurements)?;
        self.generate_html(stats)?;

        Ok(())
    }

    fn generate_html(&self, stats: SightglassStats) -> anyhow::Result<()> {
        let mut env = minijinja::Environment::new();
        env.add_template("report", TEMPLATE)?;
        env.add_filter("floatfmt", |v: f64| format!("{v:0.2}"));
        env.add_filter("intfmt", |v: f64| format!("{v:.0}"));
        env.add_filter("cyclefmt", |v: f64| {
            let v = v.abs();
            if v >= 1_000_000_000.0 {
                format!("{:.1}B", v / 1_000_000_000.0)
            } else if v >= 1_000_000.0 {
                format!("{:.1}M", v / 1_000_000.0)
            } else if v >= 1_000.0 {
                format!("{:.1}K", v / 1_000.0)
            } else {
                format!("{:.0}", v)
            }
        });

        let template = env.get_template("report")?;

        let confidence_pct = (1.0 - self.significance_level) * 100.0;
        let ctx = minijinja::context!(
            stats => stats,
            significance_level => self.significance_level,
            confidence_pct => confidence_pct,
        );

        let mut f = std::fs::File::create(&self.output_path)?;
        f.write_all(template.render(ctx)?.as_bytes())?;

        Ok(())
    }
}
