//! Static PCA metrics: entity counts and the static instruction mix, computed
//! by parsing the Wasm module.

use super::category::Category;
use super::Counts;
use anyhow::{bail, Context, Result};
use wasmparser::Payload;

/// Compute the static metrics for `wasm`, accumulating them into `counts`.
pub(crate) fn static_metrics(wasm: &[u8], counts: &mut Counts) -> Result<()> {
    eprintln!("> static metrics");
    let mut parser = wasmparser::Parser::new(0);
    parser.set_features(wasmparser::WasmFeatures::all());

    for payload in parser.parse_all(wasm) {
        match payload.context("failed to parse Wasm")? {
            // Entity counts.
            Payload::TypeSection(s) => counts.type_count += u64::from(s.count()),
            Payload::CoreTypeSection(s) => counts.type_count += u64::from(s.count()),
            Payload::FunctionSection(s) => counts.function_count += u64::from(s.count()),
            Payload::TableSection(s) => counts.table_count += u64::from(s.count()),
            Payload::MemorySection(s) => counts.memory_count += u64::from(s.count()),
            Payload::TagSection(s) => counts.tag_count += u64::from(s.count()),
            Payload::GlobalSection(s) => counts.global_count += u64::from(s.count()),
            Payload::ElementSection(s) => counts.elem_segment_count += u64::from(s.count()),
            Payload::DataSection(s) => counts.data_segment_count += u64::from(s.count()),
            Payload::ModuleSection { .. } => counts.module_count += 1,
            Payload::InstanceSection(s) => counts.core_instance_count += u64::from(s.count()),
            Payload::ComponentSection { .. } => counts.component_count += 1,
            Payload::ComponentInstanceSection(s) => {
                counts.component_instance_count += u64::from(s.count())
            }
            Payload::ComponentTypeSection(s) => counts.component_type_count += u64::from(s.count()),
            Payload::ComponentCanonicalSection(s) => {
                counts.component_canon_function_count += u64::from(s.count())
            }

            // The static instruction mix.
            Payload::CodeSectionEntry(body) => {
                let mut reader = body.get_operators_reader()?;
                while !reader.eof() {
                    let op = reader.read()?;
                    counts.static_insts[Category::for_op(&op)? as usize] += 1;
                    counts.total_static_insts += 1;
                }
            }

            // Sections that don't contribute to the metrics we track.
            Payload::Version { .. }
            | Payload::ImportSection(_)
            | Payload::ExportSection(_)
            | Payload::StartSection { .. }
            | Payload::DataCountSection { .. }
            | Payload::CodeSectionStart { .. }
            | Payload::CustomSection(_)
            | Payload::ComponentImportSection(_)
            | Payload::ComponentExportSection(_)
            | Payload::ComponentAliasSection(_)
            | Payload::ComponentStartSection { .. }
            | Payload::UnknownSection { .. }
            | Payload::End(_) => {}

            // `Payload` is `#[non_exhaustive]`, so we must handle unknown
            // payloads; reaching this means a section kind we don't know about.
            payload => bail!("unknown Wasm payload: {payload:?}"),
        }
    }

    Ok(())
}
