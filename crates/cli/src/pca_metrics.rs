//! Capture benchmark metrics for principal component analysis (PCA).

mod category;
mod dynamic_metrics;
mod static_metrics;

use crate::suite::BenchmarkOrSuite;
use anyhow::{Context, Result};
use category::{Category, NUM_CATEGORIES};
use clap::Parser;
use dynamic_metrics::{dynamic_metrics, make_engine};
use serde::Serialize;
use sightglass_build::get_engine_filename;
use static_metrics::static_metrics;
use std::path::{Path, PathBuf};

/// Capture benchmark metrics for principal component analysis (PCA).
#[derive(Debug, Parser)]
#[command(name = "pca-metrics")]
pub struct PcaMetricsCommand {
    /// The optional file path to write output to. Writes output to stdout if
    /// omitted.
    #[arg(long, short)]
    output: Option<PathBuf>,

    /// Optionally bound each benchmark's execution to this many units of fuel.
    ///
    /// When given, benchmarks are compiled and run with fuel metering and may
    /// stop early once their fuel is exhausted. When omitted, benchmarks run to
    /// completion.
    ///
    /// This primarily exists to make testing this command easier, and shouldn't
    /// be used when doing full PCA.
    #[arg(long)]
    fuel: Option<u64>,

    /// The benchmark engine with which to run Callgrind measurements.
    ///
    /// Defaults to the Wasmtime engine library in this repository.
    #[arg(long, short)]
    engine: Option<PathBuf>,

    /// The Wasm benchmarks whose PCA metrics should be taken.
    inputs: Vec<BenchmarkOrSuite>,
}

impl PcaMetricsCommand {
    pub fn execute(&self) -> Result<()> {
        let benchmark_engine = self.benchmark_engine_path()?;
        let stdout;
        let output: Box<dyn std::io::Write> = match &self.output {
            Some(path) => {
                let file = std::fs::File::create(path)
                    .with_context(|| format!("failed to create file: {}", path.display()))?;
                Box::new(std::io::BufWriter::new(file)) as _
            }
            None => {
                stdout = std::io::stdout();
                Box::new(stdout.lock()) as _
            }
        };

        let mut csv = csv::WriterBuilder::new()
            .has_headers(true)
            .from_writer(output);

        let wasm_files: Vec<_> = self
            .inputs
            .iter()
            .flat_map(|f| f.paths())
            .map(|p| p.display().to_string())
            .collect();

        let engine = make_engine(self.fuel.is_some())?;

        let mut error = None;
        for f in wasm_files {
            eprintln!("Gathering PCA metrics for {f}");
            match pca_metrics(&f, &benchmark_engine, &engine, self.fuel) {
                Ok(metrics) => {
                    csv.serialize(metrics)
                        .context("failed to serialize PCA metrics into CSV file")?;
                }
                // Collecting metrics for a misbehaving benchmark might
                // fail. Log it for now and move on, rather than aborting the
                // whole run and throwing away the metrics we have successfully
                // collected.
                Err(e) => {
                    eprintln!("failed to compute PCA metrics for {f}, skipping: {e:?}");
                    log::warn!("failed to compute PCA metrics for {f}, skipping: {e:?}");
                    match error.take() {
                        None => error = Some(e),
                        Some(root) => error = Some(root.context(e)),
                    }
                }
            }
        }

        csv.flush().context("failed to flush data to CSV file")?;

        match error {
            None => Ok(()),
            Some(e) => Err(e),
        }
    }

    fn benchmark_engine_path(&self) -> Result<PathBuf> {
        let engine = self.engine.clone().unwrap_or_else(default_engine_path);
        anyhow::ensure!(
            engine.exists(),
            "invalid path to engine: {}",
            engine.display()
        );
        Ok(engine)
    }
}

fn default_engine_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("engines/wasmtime")
        .join(get_engine_filename())
}

/// Compute the full set of PCA metrics for a single benchmark: static entity
/// counts, the static instruction mix, and the dynamic instruction mix gathered
/// by instrumenting and running the benchmark once.
fn pca_metrics<'a>(
    benchmark: &'a str,
    benchmark_engine: &Path,
    engine: &wasmtime::Engine,
    fuel: Option<u64>,
) -> Result<PcaMetrics<'a>> {
    let wasm = std::fs::read(benchmark)
        .with_context(|| format!("failed to read benchmark: {benchmark}"))?;

    let mut counts = Counts::default();
    static_metrics(&wasm, &mut counts).context("failed to compute static metrics")?;

    // The benchmark reads its workload from its own directory, so preopen that
    // as the working directory when we run it.
    let working_dir = Path::new(benchmark)
        .parent()
        .unwrap_or_else(|| Path::new("."));
    dynamic_metrics(
        &wasm,
        Path::new(benchmark),
        &mut counts,
        benchmark_engine,
        engine,
        working_dir,
        fuel,
    )
    .context("failed to compute dynamic metrics")?;

    Ok(PcaMetrics::from_counts(benchmark, &counts))
}

/// Raw, un-normalized metric accumulators for a single benchmark.
#[derive(Default)]
struct Counts {
    // Static entity counts.
    type_count: u64,
    tag_count: u64,
    global_count: u64,
    memory_count: u64,
    table_count: u64,
    data_segment_count: u64,
    elem_segment_count: u64,
    function_count: u64,
    module_count: u64,
    core_instance_count: u64,
    component_count: u64,
    component_instance_count: u64,
    component_type_count: u64,
    component_canon_function_count: u64,

    // Static instruction mix: the number of instructions of each category that
    // appear in the module's code, plus the total instruction count.
    total_static_insts: u64,
    static_insts: [u64; NUM_CATEGORIES],

    // Dynamic instruction mix: the number of instructions of each category that
    // were executed when the benchmark ran, plus the total executed.
    total_dynamic_insts: u64,
    dynamic_insts: [u64; NUM_CATEGORIES],

    // Callgrind-based native execution counts.
    instructions_retired: u64,
    conditional_branch_misses: u64,
    conditional_branches: u64,
    indirect_branch_misses: u64,
    indirect_branches: u64,
    data_reads: u64,
    data_writes: u64,
    l1_dcache_read_misses: u64,
    l1_dcache_write_misses: u64,
    ll_dcache_read_misses: u64,
    ll_dcache_write_misses: u64,
    l1_icache_misses: u64,
    ll_icache_misses: u64,
}

/// PCA metrics.
///
/// Entity counts are normalized as `1.0 / (count + 1.0)`, and the instruction
/// and Callgrind-derived fields are emitted as ratios over their respective
/// totals.
#[derive(Serialize)]
struct PcaMetrics<'a> {
    benchmark: &'a str,

    // Static entity counts.
    type_count: f64,
    tag_count: f64,
    global_count: f64,
    memory_count: f64,
    table_count: f64,
    data_segment_count: f64,
    elem_segment_count: f64,
    function_count: f64,
    total_inst_count: f64,
    module_count: f64,
    core_instance_count: f64,
    component_count: f64,
    component_instance_count: f64,
    component_type_count: f64,
    component_canon_function_count: f64,

    // Static instruction mix.
    static_unreachable_inst_ratio: f64,
    static_nop_inst_ratio: f64,
    static_control_branch_inst_ratio: f64,
    static_control_call_inst_ratio: f64,
    static_control_exception_inst_ratio: f64,
    static_control_stack_switch_inst_ratio: f64,
    static_local_variable_inst_ratio: f64,
    static_global_variable_inst_ratio: f64,
    static_atomic_global_variable_inst_ratio: f64,
    static_table_inst_ratio: f64,
    static_atomic_table_inst_ratio: f64,
    static_memory_size_inst_ratio: f64,
    static_memory_grow_inst_ratio: f64,
    static_memory_load_inst_ratio: f64,
    static_memory_store_inst_ratio: f64,
    static_memory_other_inst_ratio: f64,
    static_ref_inst_ratio: f64,
    static_i31_inst_ratio: f64,
    static_aggregate_new_inst_ratio: f64,
    static_aggregate_get_inst_ratio: f64,
    static_aggregate_set_inst_ratio: f64,
    static_atomic_aggregate_inst_ratio: f64,
    static_numeric_integer_inst_ratio: f64,
    static_numeric_float_inst_ratio: f64,
    static_vector_inst_ratio: f64,
    static_select_inst_ratio: f64,

    // Dynamic instruction mix.
    dynamic_total_inst_count: f64,
    dynamic_unreachable_inst_ratio: f64,
    dynamic_nop_inst_ratio: f64,
    dynamic_control_branch_inst_ratio: f64,
    dynamic_control_call_inst_ratio: f64,
    dynamic_control_exception_inst_ratio: f64,
    dynamic_control_stack_switch_inst_ratio: f64,
    dynamic_local_variable_inst_ratio: f64,
    dynamic_global_variable_inst_ratio: f64,
    dynamic_atomic_global_variable_inst_ratio: f64,
    dynamic_table_inst_ratio: f64,
    dynamic_atomic_table_inst_ratio: f64,
    dynamic_memory_size_inst_ratio: f64,
    dynamic_memory_grow_inst_ratio: f64,
    dynamic_memory_load_inst_ratio: f64,
    dynamic_memory_store_inst_ratio: f64,
    dynamic_memory_other_inst_ratio: f64,
    dynamic_ref_inst_ratio: f64,
    dynamic_i31_inst_ratio: f64,
    dynamic_aggregate_new_inst_ratio: f64,
    dynamic_aggregate_get_inst_ratio: f64,
    dynamic_aggregate_set_inst_ratio: f64,
    dynamic_atomic_aggregate_inst_ratio: f64,
    dynamic_numeric_integer_inst_ratio: f64,
    dynamic_numeric_float_inst_ratio: f64,
    dynamic_vector_inst_ratio: f64,
    dynamic_select_inst_ratio: f64,

    // Callgrind-based dynamic ratios.
    wasm_insts_per_native_inst: f64,
    conditional_branch_misses: f64,
    conditional_branches: f64,
    indirect_branch_misses: f64,
    indirect_branches: f64,
    l1_dcache_read_misses: f64,
    l1_dcache_write_misses: f64,
    ll_dcache_read_misses: f64,
    ll_dcache_write_misses: f64,
    l1_icache_misses: f64,
    ll_icache_misses: f64,
}

impl<'a> PcaMetrics<'a> {
    fn from_counts(benchmark: &'a str, c: &Counts) -> Self {
        // Normalize an entity count.
        let n = |count: u64| 1.0 / (count as f64 + 1.0);
        // Normalize a static instruction-mix ratio.
        let s = |cat: Category| ratio(c.static_insts[cat as usize], c.total_static_insts);
        // Normalize a dynamic instruction-mix ratio.
        let d = |cat: Category| ratio(c.dynamic_insts[cat as usize], c.total_dynamic_insts);

        PcaMetrics {
            benchmark,

            type_count: n(c.type_count),
            tag_count: n(c.tag_count),
            global_count: n(c.global_count),
            memory_count: n(c.memory_count),
            table_count: n(c.table_count),
            data_segment_count: n(c.data_segment_count),
            elem_segment_count: n(c.elem_segment_count),
            function_count: n(c.function_count),
            total_inst_count: n(c.total_static_insts),
            module_count: n(c.module_count),
            core_instance_count: n(c.core_instance_count),
            component_count: n(c.component_count),
            component_instance_count: n(c.component_instance_count),
            component_type_count: n(c.component_type_count),
            component_canon_function_count: n(c.component_canon_function_count),

            static_unreachable_inst_ratio: s(Category::Unreachable),
            static_nop_inst_ratio: s(Category::Nop),
            static_control_branch_inst_ratio: s(Category::ControlBranch),
            static_control_call_inst_ratio: s(Category::ControlCall),
            static_control_exception_inst_ratio: s(Category::ControlException),
            static_control_stack_switch_inst_ratio: s(Category::ControlStackSwitch),
            static_local_variable_inst_ratio: s(Category::LocalVariable),
            static_global_variable_inst_ratio: s(Category::GlobalVariable),
            static_atomic_global_variable_inst_ratio: s(Category::AtomicGlobalVariable),
            static_table_inst_ratio: s(Category::Table),
            static_atomic_table_inst_ratio: s(Category::AtomicTable),
            static_memory_size_inst_ratio: s(Category::MemorySize),
            static_memory_grow_inst_ratio: s(Category::MemoryGrow),
            static_memory_load_inst_ratio: s(Category::MemoryLoad),
            static_memory_store_inst_ratio: s(Category::MemoryStore),
            static_memory_other_inst_ratio: s(Category::MemoryOther),
            static_ref_inst_ratio: s(Category::Ref),
            static_i31_inst_ratio: s(Category::I31),
            static_aggregate_new_inst_ratio: s(Category::AggregateNew),
            static_aggregate_get_inst_ratio: s(Category::AggregateGet),
            static_aggregate_set_inst_ratio: s(Category::AggregateSet),
            static_atomic_aggregate_inst_ratio: s(Category::AtomicAggregate),
            static_numeric_integer_inst_ratio: s(Category::NumericInteger),
            static_numeric_float_inst_ratio: s(Category::NumericFloat),
            static_vector_inst_ratio: s(Category::Vector),
            static_select_inst_ratio: s(Category::Select),

            dynamic_total_inst_count: c.total_dynamic_insts as f64,
            dynamic_unreachable_inst_ratio: d(Category::Unreachable),
            dynamic_nop_inst_ratio: d(Category::Nop),
            dynamic_control_branch_inst_ratio: d(Category::ControlBranch),
            dynamic_control_call_inst_ratio: d(Category::ControlCall),
            dynamic_control_exception_inst_ratio: d(Category::ControlException),
            dynamic_control_stack_switch_inst_ratio: d(Category::ControlStackSwitch),
            dynamic_local_variable_inst_ratio: d(Category::LocalVariable),
            dynamic_global_variable_inst_ratio: d(Category::GlobalVariable),
            dynamic_atomic_global_variable_inst_ratio: d(Category::AtomicGlobalVariable),
            dynamic_table_inst_ratio: d(Category::Table),
            dynamic_atomic_table_inst_ratio: d(Category::AtomicTable),
            dynamic_memory_size_inst_ratio: d(Category::MemorySize),
            dynamic_memory_grow_inst_ratio: d(Category::MemoryGrow),
            dynamic_memory_load_inst_ratio: d(Category::MemoryLoad),
            dynamic_memory_store_inst_ratio: d(Category::MemoryStore),
            dynamic_memory_other_inst_ratio: d(Category::MemoryOther),
            dynamic_ref_inst_ratio: d(Category::Ref),
            dynamic_i31_inst_ratio: d(Category::I31),
            dynamic_aggregate_new_inst_ratio: d(Category::AggregateNew),
            dynamic_aggregate_get_inst_ratio: d(Category::AggregateGet),
            dynamic_aggregate_set_inst_ratio: d(Category::AggregateSet),
            dynamic_atomic_aggregate_inst_ratio: d(Category::AtomicAggregate),
            dynamic_numeric_integer_inst_ratio: d(Category::NumericInteger),
            dynamic_numeric_float_inst_ratio: d(Category::NumericFloat),
            dynamic_vector_inst_ratio: d(Category::Vector),
            dynamic_select_inst_ratio: d(Category::Select),

            wasm_insts_per_native_inst: ratio(c.total_dynamic_insts, c.instructions_retired),
            conditional_branch_misses: ratio(c.conditional_branch_misses, c.conditional_branches),
            conditional_branches: ratio(c.conditional_branches, c.instructions_retired),
            indirect_branch_misses: ratio(c.indirect_branch_misses, c.indirect_branches),
            indirect_branches: ratio(c.indirect_branches, c.instructions_retired),
            l1_dcache_read_misses: ratio(c.l1_dcache_read_misses, c.data_reads),
            l1_dcache_write_misses: ratio(c.l1_dcache_write_misses, c.data_writes),
            ll_dcache_read_misses: ratio(c.ll_dcache_read_misses, c.data_reads),
            ll_dcache_write_misses: ratio(c.ll_dcache_write_misses, c.data_writes),
            l1_icache_misses: ratio(c.l1_icache_misses, c.instructions_retired),
            ll_icache_misses: ratio(c.ll_icache_misses, c.instructions_retired),
        }
    }
}

/// Normalize a category's instruction count against the total.
fn ratio(count: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        count as f64 / total as f64
    }
}

#[cfg(test)]
mod tests {
    use super::{Counts, PcaMetrics};

    #[test]
    fn callgrind_ratios_are_normalized_from_counts() {
        let metrics = PcaMetrics::from_counts(
            "benchmark.wasm",
            &Counts {
                total_dynamic_insts: 250,
                instructions_retired: 500,
                conditional_branch_misses: 5,
                conditional_branches: 20,
                indirect_branch_misses: 3,
                indirect_branches: 10,
                data_reads: 40,
                data_writes: 50,
                l1_dcache_read_misses: 4,
                l1_dcache_write_misses: 5,
                ll_dcache_read_misses: 2,
                ll_dcache_write_misses: 1,
                l1_icache_misses: 25,
                ll_icache_misses: 10,
                ..Counts::default()
            },
        );

        assert_eq!(metrics.wasm_insts_per_native_inst, 0.5);
        assert_eq!(metrics.conditional_branch_misses, 0.25);
        assert_eq!(metrics.conditional_branches, 0.04);
        assert_eq!(metrics.indirect_branch_misses, 0.3);
        assert_eq!(metrics.indirect_branches, 0.02);
        assert_eq!(metrics.l1_dcache_read_misses, 0.1);
        assert_eq!(metrics.l1_dcache_write_misses, 0.1);
        assert_eq!(metrics.ll_dcache_read_misses, 0.05);
        assert_eq!(metrics.ll_dcache_write_misses, 0.02);
        assert_eq!(metrics.l1_icache_misses, 0.05);
        assert_eq!(metrics.ll_icache_misses, 0.02);
    }
}
