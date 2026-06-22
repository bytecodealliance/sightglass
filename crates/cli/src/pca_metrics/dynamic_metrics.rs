//! Dynamic PCA metrics: the dynamically executed instruction mix, cache
//! behavior, etc... Gathered by instrumenting the module to count how many
//! instructions of each category it executes and then running it once.

mod component;

use super::category::{Category, NUM_CATEGORIES};
use super::Counts;
use anyhow::{bail, Context, Result};
use sightglass_data::{Measurement, Phase};
use std::path::Path;
#[cfg(all(target_os = "linux", feature = "callgrind"))]
use std::process::{Command, Stdio};
use wasmparser::Payload;

/// The name prefix used for the exported core functions that return how many
/// times each category of instruction was executed by an instrumented core
/// module (used when not flushing to the host; see `instrument`).
const COUNTER_PREFIX: &str = "__pca_count_";

/// The imported instance and function a component core module flushes its
/// counts through (and that `run_component` supplies from the host).
const PCA_INSTANCE: &str = "sightglass-pca";
const INCREMENT_FN: &str = "increment-instruction-count";

/// Functions exported under these conventional names are the canonical ABI
/// `realloc`, which runs with the component's "may leave" flag cleared and so
/// cannot call the host `increment-instruction-count`. When flushing, we still
/// count their instructions into the per-category globals but never flush from
/// inside them; a later, ordinary function flushes the accumulated counts.
const NO_FLUSH_EXPORTS: &[&str] = &["realloc", "cabi_realloc"];

/// Compute the dynamic instruction mix for `wasm`, accumulating it into
/// `counts`, by instrumenting and running the benchmark once.
pub(crate) fn dynamic_metrics(
    wasm: &[u8],
    benchmark: &Path,
    counts: &mut Counts,
    benchmark_engine: &Path,
    engine: &wasmtime::Engine,
    working_dir: &Path,
    fuel: Option<u64>,
) -> Result<()> {
    eprintln!("> dynamic metrics");

    if is_component(wasm) {
        let instrumented =
            component::instrument_component(wasm).context("failed to instrument component")?;
        run_instrumented_component(&instrumented, counts, engine, working_dir, fuel)
            .context("failed to run instrumented component")?;
    } else {
        let instrumented = instrument_core_module(wasm, false)
            .context("failed to instrument Wasm")?
            .finish();
        run_instrumented_core_module(&instrumented, counts, engine, working_dir, fuel)
            .context("failed to run instrumented Wasm")?;
    }

    // Fuel-capped runs are intentionally partial and only used for quick tests;
    // skip Callgrind because its too slow for this use case.
    if fuel.is_none() {
        callgrind_metrics(benchmark, counts, benchmark_engine, working_dir)
            .context("failed to collect Callgrind metrics")?;
    }

    Ok(())
}

#[cfg(all(target_os = "linux", feature = "callgrind"))]
fn callgrind_metrics(
    benchmark: &Path,
    counts: &mut Counts,
    benchmark_engine: &Path,
    working_dir: &Path,
) -> Result<()> {
    eprintln!("> callgrind metrics");
    let this_exe = std::env::current_exe().context("failed to get current executable")?;
    let output = Command::new(&this_exe)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("benchmark")
        .arg("--measure")
        .arg("callgrind")
        .arg("--engine")
        .arg(benchmark_engine)
        .arg("--raw")
        .arg("--output-format")
        .arg("json")
        .arg("--processes")
        .arg("1")
        .arg("--iterations-per-process")
        .arg("1")
        .arg("--benchmark-phase")
        .arg("execution")
        .arg("--working-dir")
        .arg(working_dir)
        .arg("--engine-flags")
        .arg("-Wgc,function-references")
        .arg("--")
        .arg(benchmark)
        .output()
        .with_context(|| {
            format!(
                "failed to spawn benchmark subprocess for {}",
                benchmark.display()
            )
        })?;

    anyhow::ensure!(
        output.status.success(),
        "callgrind benchmark subprocess failed for {}: {}",
        benchmark.display(),
        String::from_utf8_lossy(&output.stderr).trim()
    );

    let measurements = serde_json::from_slice::<Vec<Measurement<'static>>>(&output.stdout)
        .with_context(|| {
            format!(
                "failed to parse callgrind benchmark output for {}",
                benchmark.display()
            )
        })?;
    accumulate_callgrind_counts(&measurements, counts);
    Ok(())
}

#[cfg(not(all(target_os = "linux", feature = "callgrind")))]
fn callgrind_metrics(
    _benchmark: &Path,
    _counts: &mut Counts,
    _benchmark_engine: &Path,
    _working_dir: &Path,
) -> Result<()> {
    Ok(())
}

fn accumulate_callgrind_counts(measurements: &[Measurement<'_>], counts: &mut Counts) {
    for measurement in measurements {
        if measurement.phase != Phase::Execution {
            continue;
        }

        match measurement.event.as_ref() {
            "instructions-retired" => counts.instructions_retired += measurement.count,
            "conditional-branch-misses" => counts.conditional_branch_misses += measurement.count,
            "conditional-branches" => counts.conditional_branches += measurement.count,
            "indirect-branch-misses" => counts.indirect_branch_misses += measurement.count,
            "indirect-branches" => counts.indirect_branches += measurement.count,
            "data-reads" => counts.data_reads += measurement.count,
            "data-writes" => counts.data_writes += measurement.count,
            "l1-dcache-read-misses" => counts.l1_dcache_read_misses += measurement.count,
            "l1-dcache-write-misses" => counts.l1_dcache_write_misses += measurement.count,
            "ll-dcache-read-misses" => counts.ll_dcache_read_misses += measurement.count,
            "ll-dcache-write-misses" => counts.ll_dcache_write_misses += measurement.count,
            "l1-icache-misses" => counts.l1_icache_misses += measurement.count,
            "ll-icache-misses" => counts.ll_icache_misses += measurement.count,
            otherwise => {
                eprintln!("ignoring unknown event: `{otherwise}`");
                log::warn!("ignoring unknown event: `{otherwise}`");
            }
        }
    }
}

/// Whether `wasm` is a component, as opposed to a core module.
fn is_component(wasm: &[u8]) -> bool {
    matches!(
        wasmparser::Parser::new(0).parse_all(wasm).next(),
        Some(Ok(Payload::Version {
            encoding: wasmparser::Encoding::Component,
            ..
        }))
    )
}

/// Run an instrumented component once and accumulate its category counters into
/// `counts`.
///
/// The component is run the way `wasm_bench_create` runs components in
/// `wasmtime`'s `bench-api`: with WASI preview 2 and the benchmarking
/// hooks. The instrumentation pushes counts to the host via a `sightglass-pca`
/// `increment-instruction-count` import as it runs, and the host tallies them
/// while benchmarking is active (between `bench.start`/`bench.end`). Because
/// the counts live in host memory rather than inside the component, we recover
/// them even when the benchmark runs out of fuel mid-run.
fn run_instrumented_component(
    component_bytes: &[u8],
    counts: &mut Counts,
    engine: &wasmtime::Engine,
    working_dir: &Path,
    fuel: Option<u64>,
) -> Result<()> {
    use wasmtime::component::{Component, Linker, ResourceTable};
    use wasmtime::{Store, StoreContextMut, Trap};
    use wasmtime_wasi::p2::bindings::sync::Command;
    use wasmtime_wasi::{DirPerms, FilePerms, WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

    struct Host {
        wasi: WasiCtx,
        table: ResourceTable,
        /// Whether we are between `bench.start` and `bench.end`.
        active: bool,
        /// Executed-instruction counts per category, pushed by the component.
        dynamic: [u64; NUM_CATEGORIES],
    }
    impl WasiView for Host {
        fn ctx(&mut self) -> WasiCtxView<'_> {
            WasiCtxView {
                ctx: &mut self.wasi,
                table: &mut self.table,
            }
        }
    }

    let component = Component::new(engine, component_bytes)?;

    // Provide WASIp2, the benchmarking hooks (which toggle the active flag), and
    // the `sightglass-pca` instance the instrumentation pushes counts to.
    let mut linker: Linker<Host> = Linker::new(engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    let mut bench = linker.instance("bench")?;
    bench.func_wrap("start", |mut cx: StoreContextMut<'_, Host>, (): ()| {
        cx.data_mut().active = true;
        Ok(())
    })?;
    bench.func_wrap("end", |mut cx: StoreContextMut<'_, Host>, (): ()| {
        cx.data_mut().active = false;
        Ok(())
    })?;
    let mut pca = linker.instance(PCA_INSTANCE)?;
    pca.func_wrap(
        INCREMENT_FN,
        |mut cx: StoreContextMut<'_, Host>, (category, count): (u32, u32)| {
            let host = cx.data_mut();
            if host.active {
                if let Some(slot) = host.dynamic.get_mut(category as usize) {
                    *slot += u64::from(count);
                }
            }
            Ok(())
        },
    )?;

    let mut builder = WasiCtxBuilder::new();
    builder.preopened_dir(working_dir, ".", DirPerms::READ, FilePerms::READ)?;
    let host = Host {
        wasi: builder.build(),
        table: ResourceTable::new(),
        active: false,
        dynamic: [0; NUM_CATEGORIES],
    };

    let mut store = Store::new(engine, host);
    if let Some(fuel) = fuel {
        store.set_fuel(fuel)?;
    }

    let instance = linker.instantiate(&mut store, &component)?;

    // Run `wasi:cli/run`'s `run`. Completing (with any exit status) and running
    // out of fuel are both acceptable stopping points: in either case the host
    // already holds the counts pushed before we stopped.
    let command = Command::new(&mut store, &instance)?;
    if let Err(e) = command.wasi_cli_run().call_run(&mut store) {
        if !(fuel.is_some() && matches!(e.downcast_ref::<Trap>(), Some(Trap::OutOfFuel))) {
            return Err(e.into());
        }
    }

    let host = store.data();
    for i in 0..NUM_CATEGORIES {
        counts.dynamic_insts[i] += host.dynamic[i];
        counts.total_dynamic_insts += host.dynamic[i];
    }

    Ok(())
}

/// Create the Wasmtime engine used to run instrumented benchmarks.
pub(crate) fn make_engine(consume_fuel: bool) -> Result<wasmtime::Engine> {
    let mut config = wasmtime::Config::new();
    config.consume_fuel(consume_fuel);
    config.compiler_inlining(true);

    // Do not verify the CLIF (enabled in debug builds; slow).
    config.cranelift_debug_verifier(false);

    // Required for running some of our benchmarks.
    config.wasm_gc(true);
    config.wasm_function_references(true);

    // Enable Wasmtime's compilation cache.
    if let Ok(cache) = wasmtime::Cache::new(wasmtime::CacheConfig::default()) {
        config.cache(Some(cache));
    }

    let engine = wasmtime::Engine::new(&config)?;
    Ok(engine)
}

/// Whether `op` ends a basic block, i.e. the *next* operator begins a new
/// straight-line run (a "leader").
///
/// Structured control (`block`/`loop`/`if`/`else`/`end`), branches, returns,
/// `unreachable`, and exception control are boundaries; plain calls are not,
/// since they return and fall through.
///
/// Note that being conservative is safe: splitting a block we didn't need to
/// still counts correctly, whereas *missing* a real boundary would let us count
/// instructions past a branch that doesn't execute.
fn is_block_boundary(op: &wasmparser::Operator<'_>) -> bool {
    use wasmparser::Operator::*;
    matches!(
        op,
        Block { .. }
            | Loop { .. }
            | If { .. }
            | Else
            | End
            | Br { .. }
            | BrIf { .. }
            | BrTable { .. }
            | Return
            | ReturnCall { .. }
            | ReturnCallIndirect { .. }
            | ReturnCallRef { .. }
            | Unreachable
            | BrOnNull { .. }
            | BrOnNonNull { .. }
            | BrOnCast { .. }
            | BrOnCastFail { .. }
            | Throw { .. }
            | ThrowRef
            | Rethrow { .. }
            | TryTable { .. }
            | Try { .. }
            | Catch { .. }
            | CatchAll
            | Delegate { .. }
    )
}

/// One basic block of a function body: a maximal straight-line run of operators.
struct BasicBlock {
    /// The operator index, within the function body, where this block starts.
    start: u32,
    /// Whether it begins at a function entry or the top of a loop, where the
    /// component instrumentation flushes its counters to the host.
    is_flush_point: bool,
    /// How many operators of each [`Category`] the block contains.
    counts: [u32; NUM_CATEGORIES],
}

/// Partition a function body into [`BasicBlock`]s, tallying operators by category.
///
/// This is shared by the core-module and component instrumentation so they both
/// increment counters once per basic block (in batch) rather than once per
/// operator.
fn basic_blocks(body: &wasmparser::FunctionBody<'_>) -> Result<Vec<BasicBlock>> {
    let mut blocks: Vec<BasicBlock> = Vec::new();
    let mut reader = body.get_operators_reader()?;
    let mut idx = 0u32;
    let mut at_leader = true;
    let mut prev_was_loop = false;
    while !reader.eof() {
        let op = reader.read()?;
        if at_leader {
            blocks.push(BasicBlock {
                start: idx,
                is_flush_point: idx == 0 || prev_was_loop,
                counts: [0; NUM_CATEGORIES],
            });
        }
        blocks.last_mut().unwrap().counts[Category::for_op(&op)? as usize] += 1;
        at_leader = is_block_boundary(&op);
        prev_was_loop = matches!(op, wasmparser::Operator::Loop { .. });
        idx += 1;
    }
    Ok(blocks)
}

/// Rewrite a core module so it counts how many instructions of each [`Category`]
/// it executes, tallying them once per basic block (in batch) rather than once
/// per operator.
///
/// The counters are appended module globals; how they are gated and read back
/// depends on `flush`:
///
/// * `flush == false` (standalone core-module benchmarks): the counters are
///   `i64`, and each batch is multiplied by an appended `is-benchmarking-active`
///   global (set to `1` after `bench.start`, `0` before `bench.end`) so only the
///   measured region is counted. One exported getter per category lets the host
///   read the totals back after the run.
///
/// * `flush == true` (components' core modules): the counters are `i32`, and
///   the module imports `sightglass-pca.increment-instruction-count` as
///   function index 0. At each function entry and loop top it pushes every
///   (used) counter to that import and resets it (so counts reach the host even
///   when we hit an out-of-fuel trap and don't finish execution) except in
///   canonical ABI `realloc` functions, which only accumulate. The host gates
///   on the active region itself.
///
/// Appended items take the end of their index spaces, so the only renumbering is
/// the `+1` function shift that flushing's prepended import requires.
fn instrument_core_module(wasm: &[u8], flush: bool) -> Result<wasm_encoder::Module> {
    use wasm_encoder::reencode::{Error as ReencodeError, Reencode};
    use wasm_encoder::{
        CodeSection, ConstExpr, DataCountSection, DataSection, ElementSection, EntityType,
        ExportKind, ExportSection, Function, FunctionSection, GlobalSection, GlobalType,
        ImportSection, Instruction, MemorySection, Module, StartSection, TableSection, TagSection,
        TypeSection, ValType,
    };

    // Shift function references by one only when flushing prepends its import.
    struct Shift(u32);
    impl Reencode for Shift {
        type Error = std::convert::Infallible;
        fn function_index(&mut self, i: u32) -> Result<u32, ReencodeError<Self::Error>> {
            Ok(i + self.0)
        }
    }
    let reencoder = &mut Shift(if flush { 1 } else { 0 });

    // When flushing, pre-scan which categories occur so we only flush counters
    // that can be non-zero.
    let mut used = [false; NUM_CATEGORIES];
    if flush {
        for payload in wasmparser::Parser::new(0).parse_all(wasm) {
            if let Payload::CodeSectionEntry(body) = payload.context("failed to parse Wasm")? {
                let mut reader = body.get_operators_reader()?;
                while !reader.eof() {
                    used[Category::for_op(&reader.read()?)? as usize] = true;
                }
            }
        }
    }

    let mut types: Option<TypeSection> = None;
    let mut imports: Option<ImportSection> = None;
    let mut functions: Option<FunctionSection> = None;
    let mut tables: Option<TableSection> = None;
    let mut memories: Option<MemorySection> = None;
    let mut tags: Option<TagSection> = None;
    let mut globals: Option<GlobalSection> = None;
    let mut exports: Option<ExportSection> = None;
    let mut start: Option<u32> = None;
    let mut elements: Option<ElementSection> = None;
    let mut data_count: Option<u32> = None;
    let mut code: Option<CodeSection> = None;
    let mut data: Option<DataSection> = None;

    // Index-space sizes (final by the code section, which follows them all). For
    // `flush == false` we also record the benchmarking-hook function indices; for
    // `flush == true`, the original indices of `realloc`-style functions that
    // must not flush.
    let mut num_types = 0u32;
    let mut imported_funcs = 0u32;
    let mut imported_globals = 0u32;
    let mut defined_funcs = 0u32;
    let mut defined_globals = 0u32;
    let mut bench_start_func = None;
    let mut bench_end_func = None;
    let mut code_entry = 0u32;
    let mut no_flush_funcs: Vec<u32> = Vec::new();

    for payload in wasmparser::Parser::new(0).parse_all(wasm) {
        match payload.context("failed to parse Wasm")? {
            Payload::TypeSection(s) => {
                num_types += s.count();
                let sec = types.get_or_insert_with(TypeSection::new);
                reencoder.parse_type_section(sec, s)?;
            }
            Payload::ImportSection(s) => {
                let sec = imports.get_or_insert_with(ImportSection::new);
                if flush {
                    // Prepend the `increment` import so it is function index 0.
                    sec.import(PCA_INSTANCE, INCREMENT_FN, EntityType::Function(num_types));
                }
                for import in s {
                    let import = import?;
                    match import.ty {
                        wasmparser::TypeRef::Func(_) => {
                            if !flush && import.module == "bench" {
                                if import.name == "start" {
                                    bench_start_func = Some(imported_funcs);
                                } else if import.name == "end" {
                                    bench_end_func = Some(imported_funcs);
                                }
                            }
                            imported_funcs += 1;
                        }
                        wasmparser::TypeRef::Global(_) => imported_globals += 1,
                        _ => {}
                    }
                    reencoder.parse_import(sec, import)?;
                }
            }
            Payload::FunctionSection(s) => {
                defined_funcs += s.count();
                let sec = functions.get_or_insert_with(FunctionSection::new);
                reencoder.parse_function_section(sec, s)?;
            }
            Payload::TableSection(s) => {
                let sec = tables.get_or_insert_with(TableSection::new);
                reencoder.parse_table_section(sec, s)?;
            }
            Payload::MemorySection(s) => {
                let sec = memories.get_or_insert_with(MemorySection::new);
                reencoder.parse_memory_section(sec, s)?;
            }
            Payload::TagSection(s) => {
                let sec = tags.get_or_insert_with(TagSection::new);
                reencoder.parse_tag_section(sec, s)?;
            }
            Payload::GlobalSection(s) => {
                defined_globals += s.count();
                let sec = globals.get_or_insert_with(GlobalSection::new);
                reencoder.parse_global_section(sec, s)?;
            }
            Payload::ExportSection(s) => {
                let sec = exports.get_or_insert_with(ExportSection::new);
                for export in s {
                    let export = export?;
                    if flush
                        && matches!(export.kind, wasmparser::ExternalKind::Func)
                        && NO_FLUSH_EXPORTS.contains(&export.name)
                    {
                        no_flush_funcs.push(export.index);
                    }
                    reencoder.parse_export(sec, export)?;
                }
            }
            Payload::StartSection { func, .. } => {
                start = Some(reencoder.function_index(func)?);
            }
            Payload::ElementSection(s) => {
                let sec = elements.get_or_insert_with(ElementSection::new);
                reencoder.parse_element_section(sec, s)?;
            }
            Payload::DataCountSection { count, .. } => {
                data_count = Some(count);
            }
            Payload::CodeSectionEntry(body) => {
                let base_global = imported_globals + defined_globals;
                let active_global = base_global + NUM_CATEGORIES as u32;
                // `realloc`-style functions accumulate counts but never flush.
                let no_flush = flush && no_flush_funcs.contains(&(imported_funcs + code_entry));
                code_entry += 1;

                // Tally instructions once per basic block (in batch): at each
                // block's top, add `count * active` to each category's counter,
                // so only instructions executed between `bench.start` and
                // `bench.end` are counted.
                let blocks = basic_blocks(&body)?;
                let mut next_block = 0;
                let mut idx = 0u32;

                let mut func = reencoder.new_function_with_parsed_locals(&body)?;
                let mut reader = body.get_operators_reader()?;
                while !reader.eof() {
                    let op = reader.read()?;

                    if next_block < blocks.len() && blocks[next_block].start == idx {
                        let block = &blocks[next_block];
                        next_block += 1;

                        // When flushing, push every counter to the host and reset
                        // it at function entries and loop tops (but not in the
                        // functions that can't leave the component).
                        if flush && block.is_flush_point && !no_flush {
                            for (cat, is_used) in used.iter().copied().enumerate() {
                                if is_used {
                                    let counter = base_global + cat as u32;
                                    func.instruction(&Instruction::I32Const(cat as i32));
                                    func.instruction(&Instruction::GlobalGet(counter));
                                    func.instruction(&Instruction::Call(0));
                                    func.instruction(&Instruction::I32Const(0));
                                    func.instruction(&Instruction::GlobalSet(counter));
                                }
                            }
                        }

                        // Tally this block's operators into the counters in batch.
                        // When flushing, count unconditionally (the host gates on
                        // the active region); otherwise multiply by `active`.
                        for cat in 0..NUM_CATEGORIES {
                            let count = block.counts[cat];
                            if count > 0 {
                                let counter = base_global + cat as u32;
                                func.instruction(&Instruction::GlobalGet(counter));
                                if flush {
                                    func.instruction(&Instruction::I32Const(count as i32));
                                    func.instruction(&Instruction::I32Add);
                                } else {
                                    func.instruction(&Instruction::GlobalGet(active_global));
                                    func.instruction(&Instruction::I64Const(i64::from(count)));
                                    func.instruction(&Instruction::I64Mul);
                                    func.instruction(&Instruction::I64Add);
                                }
                                func.instruction(&Instruction::GlobalSet(counter));
                            }
                        }
                    }

                    // Standalone mode gates locally: toggle the active flag around
                    // the benchmarking hooks.
                    if flush {
                        func.instruction(&reencoder.instruction(op)?);
                    } else {
                        let call_target = match &op {
                            wasmparser::Operator::Call { function_index } => Some(*function_index),
                            _ => None,
                        };
                        if bench_end_func.is_some() && call_target == bench_end_func {
                            func.instruction(&Instruction::I64Const(0));
                            func.instruction(&Instruction::GlobalSet(active_global));
                        }
                        func.instruction(&reencoder.instruction(op)?);
                        if bench_start_func.is_some() && call_target == bench_start_func {
                            func.instruction(&Instruction::I64Const(1));
                            func.instruction(&Instruction::GlobalSet(active_global));
                        }
                    }
                    idx += 1;
                }
                code.get_or_insert_with(CodeSection::new).function(&func);
            }
            Payload::DataSection(s) => {
                let sec = data.get_or_insert_with(DataSection::new);
                reencoder.parse_data_section(sec, s)?;
            }

            // The module header and sections that need no rewriting. Code
            // section entries are handled above.
            Payload::Version { .. }
            | Payload::CustomSection(_)
            | Payload::CodeSectionStart { .. }
            | Payload::End(_) => {}

            // Component-model sections (which we can't instrument) and any
            // unknown/future payloads.
            payload => bail!("unsupported Wasm payload: {payload:?}"),
        }
    }

    let base_global = imported_globals + defined_globals;

    // Append the one type we need: `(i32, i32) -> ()` for the `increment` import
    // (flush), or `() -> i64` for the counter getters (standalone).
    {
        let ty = types.get_or_insert_with(TypeSection::new).ty();
        if flush {
            ty.function([ValType::I32, ValType::I32], []);
        } else {
            ty.function([], [ValType::I64]);
        }
    }

    // Make sure the `increment` import exists even if the module had no imports.
    if flush && imports.is_none() {
        let mut sec = ImportSection::new();
        sec.import(PCA_INSTANCE, INCREMENT_FN, EntityType::Function(num_types));
        imports = Some(sec);
    }

    // Append the counter globals: `i32` when flushing, or `i64` plus the active
    // flag and one exported getter per category when not.
    if flush {
        let i32_global = GlobalType {
            val_type: ValType::I32,
            mutable: true,
            shared: false,
        };
        let globals = globals.get_or_insert_with(GlobalSection::new);
        for _ in 0..NUM_CATEGORIES {
            globals.global(i32_global, &ConstExpr::i32_const(0));
        }
    } else {
        let i64_global = GlobalType {
            val_type: ValType::I64,
            mutable: true,
            shared: false,
        };
        let getter_type = num_types;
        let getter_func_base = imported_funcs + defined_funcs;
        let globals = globals.get_or_insert_with(GlobalSection::new);
        let functions = functions.get_or_insert_with(FunctionSection::new);
        let exports = exports.get_or_insert_with(ExportSection::new);
        let code = code.get_or_insert_with(CodeSection::new);
        for i in 0..NUM_CATEGORIES as u32 {
            // The counter global itself, plus an exported getter the host calls
            // to read it back after the run.
            globals.global(i64_global, &ConstExpr::i64_const(0));
            functions.function(getter_type);
            let mut getter = Function::new([]);
            getter.instruction(&Instruction::GlobalGet(base_global + i));
            getter.instruction(&Instruction::End);
            code.function(&getter);
            exports.export(
                &format!("{COUNTER_PREFIX}{i}"),
                ExportKind::Func,
                getter_func_base + i,
            );
        }
        // The `is-benchmarking-active` global (index base_global + NUM_CATEGORIES).
        globals.global(i64_global, &ConstExpr::i64_const(0));
    }

    // Reassemble the module, emitting sections in the canonical order.
    let mut module = Module::new();
    if let Some(s) = &types {
        module.section(s);
    }
    if let Some(s) = &imports {
        module.section(s);
    }
    if let Some(s) = &functions {
        module.section(s);
    }
    if let Some(s) = &tables {
        module.section(s);
    }
    if let Some(s) = &memories {
        module.section(s);
    }
    if let Some(s) = &tags {
        module.section(s);
    }
    if let Some(s) = &globals {
        module.section(s);
    }
    if let Some(s) = &exports {
        module.section(s);
    }
    if let Some(function_index) = start {
        module.section(&StartSection { function_index });
    }
    if let Some(s) = &elements {
        module.section(s);
    }
    if let Some(count) = data_count {
        module.section(&DataCountSection { count });
    }
    if let Some(s) = &code {
        module.section(s);
    }
    if let Some(s) = &data {
        module.section(s);
    }

    Ok(module)
}

/// Run the instrumented module once and accumulate its category counters into
/// `counts`.
fn run_instrumented_core_module(
    instrumented: &[u8],
    counts: &mut Counts,
    engine: &wasmtime::Engine,
    working_dir: &Path,
    fuel: Option<u64>,
) -> Result<()> {
    use wasmtime::{Caller, Linker, Module, Store, Trap};
    use wasmtime_wasi::p1::{self, WasiP1Ctx};
    use wasmtime_wasi::{DirPerms, FilePerms, I32Exit, WasiCtxBuilder};

    let module = Module::new(engine, instrumented)?;

    // Provide real WASI and the benchmarking hooks; together these cover every
    // import a benchmark needs.
    let mut linker: Linker<WasiP1Ctx> = Linker::new(engine);
    p1::add_to_linker_sync(&mut linker, |cx| cx)?;
    linker.func_wrap("bench", "start", |_: Caller<'_, WasiP1Ctx>| {})?;
    linker.func_wrap("bench", "end", |_: Caller<'_, WasiP1Ctx>| {})?;

    // Configure the WASI context the way `wasm_bench_create` does in
    // `wasmtime`'s `bench-api`: preopen the benchmark's directory so it can read
    // its workload, and forward the small-workload environment variable.
    let mut builder = WasiCtxBuilder::new();
    builder.preopened_dir(working_dir, ".", DirPerms::READ, FilePerms::READ)?;
    let wasi = builder.build_p1();

    let mut store = Store::new(engine, wasi);
    if let Some(fuel) = fuel {
        store.set_fuel(fuel)?;
    }
    let instance = linker.instantiate(&mut store, &module)?;

    // Run the benchmark's entry point. Two non-error outcomes besides a normal
    // return: a clean `proc_exit(0)` surfaces as an `I32Exit`, and when a fuel
    // budget is set the benchmark may legitimately run out of fuel.
    let start = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
    if let Err(e) = start.call(&mut store, ()) {
        if let Some(exit) = e.downcast_ref::<I32Exit>() {
            if exit.0 != 0 {
                bail!("benchmark exited with non-zero code {}", exit.0);
            }
        } else if fuel.is_some() && matches!(e.downcast_ref::<Trap>(), Some(Trap::OutOfFuel)) {
            // Ran out of fuel; the counts gathered so far are what we want.
        } else {
            return Err(e.into());
        }
    }

    // Reading a counter calls an exported Wasm function, which itself consumes
    // fuel, so top the store back up first if we were running with a budget (the
    // benchmark may have used all of it).
    if fuel.is_some() {
        store.set_fuel(NUM_CATEGORIES as u64 * 1024)?;
    }

    // Read each category's execution count back via its exported getter.
    let mut total = 0u64;
    for i in 0..NUM_CATEGORIES {
        let getter =
            instance.get_typed_func::<(), i64>(&mut store, &format!("{COUNTER_PREFIX}{i}"))?;
        let count = getter.call(&mut store, ())? as u64;
        counts.dynamic_insts[i] += count;
        total += count;
    }
    counts.total_dynamic_insts += total;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::accumulate_callgrind_counts;
    use crate::pca_metrics::Counts;
    use sightglass_data::{Engine, Measurement, Phase};
    use std::borrow::Cow;

    #[test]
    fn accumulates_callgrind_measurements_by_event_name() {
        let measurements = vec![
            measurement(Phase::Compilation, "instructions-retired", 999),
            measurement(Phase::Execution, "instructions-retired", 200),
            measurement(Phase::Execution, "conditional-branch-misses", 3),
            measurement(Phase::Execution, "conditional-branches", 12),
            measurement(Phase::Execution, "indirect-branch-misses", 1),
            measurement(Phase::Execution, "indirect-branches", 5),
            measurement(Phase::Execution, "data-reads", 20),
            measurement(Phase::Execution, "data-writes", 10),
            measurement(Phase::Execution, "l1-dcache-read-misses", 4),
            measurement(Phase::Execution, "l1-dcache-write-misses", 2),
            measurement(Phase::Execution, "ll-dcache-read-misses", 1),
            measurement(Phase::Execution, "ll-dcache-write-misses", 1),
            measurement(Phase::Execution, "l1-icache-misses", 8),
            measurement(Phase::Execution, "ll-icache-misses", 6),
        ];

        let mut counts = Counts::default();
        accumulate_callgrind_counts(&measurements, &mut counts);

        assert_eq!(counts.instructions_retired, 200);
        assert_eq!(counts.conditional_branch_misses, 3);
        assert_eq!(counts.conditional_branches, 12);
        assert_eq!(counts.indirect_branch_misses, 1);
        assert_eq!(counts.indirect_branches, 5);
        assert_eq!(counts.data_reads, 20);
        assert_eq!(counts.data_writes, 10);
        assert_eq!(counts.l1_dcache_read_misses, 4);
        assert_eq!(counts.l1_dcache_write_misses, 2);
        assert_eq!(counts.ll_dcache_read_misses, 1);
        assert_eq!(counts.ll_dcache_write_misses, 1);
        assert_eq!(counts.l1_icache_misses, 8);
        assert_eq!(counts.ll_icache_misses, 6);
    }

    fn measurement(phase: Phase, event: &'static str, count: u64) -> Measurement<'static> {
        Measurement {
            arch: Cow::Borrowed("x86_64"),
            engine: Engine {
                name: Cow::Borrowed("engine"),
                flags: None,
            },
            wasm: Cow::Borrowed("benchmark.wasm"),
            process: 0,
            iteration: 0,
            phase,
            event: Cow::Borrowed(event),
            count,
        }
    }
}
