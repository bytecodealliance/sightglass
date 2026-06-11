//! Component-model support for dynamic PCA metrics.
//!
//! Counting executed instructions inside a component is harder than inside a
//! core module: globals can't be exported across a component boundary,
//! out-of-fuel traps leave the "may enter" flag unset so attempts to call
//! functions (e.g. to read fuel counts) after trap will always re-trap, and the
//! benchmarking hooks (`bench.start`/`bench.end`) are lowered imports that the
//! leaf core modules might not import or otherwise be able to observe directly.
//!
//! Instead, an instrumented component *pushes* its counts to the host as it
//! runs:
//!
//! * The root component (and every sub-component) imports a `sightglass-pca`
//!   instance exporting `increment-instruction-count: func(category, count)`.
//!   The host (see [`super::run_component`]) tallies those pushes, but only
//!   while benchmarking is active: it flips an "active" flag in its own
//!   `bench.start`/`bench.end` implementations.
//!
//! * Each core module keeps one defined `i32` counter global per category,
//!   incremented in batch at the top of every basic block.
//!
//! * At the start of each function and the top of each loop, a core module
//!   flushes those globals to the host via `increment-instruction-count` and
//!   resets them to zero. (Functions used as the canonical ABI `realloc` run
//!   with the component's "may leave" flag cleared, so they only accumulate
//!   into the globals and never flush; the next function flushes their counts
//!   later.)
//!
//! Because the counts live in host memory, we recover them even when the
//! benchmark runs out of fuel and traps mid-run.

use super::{INCREMENT_FN, PCA_INSTANCE};
use anyhow::Result;

/// An empty set of canonical lowering options (our scalar `increment` import
/// needs no memory or realloc).
fn no_opts() -> impl ExactSizeIterator<Item = wasm_encoder::CanonicalOption> {
    core::iter::empty()
}

/// Emit the `sightglass-pca` instance type into `types` (as the next component
/// type) and import it into `imports` (as the next component instance).
///
/// Both the root and every sub-component import this instance; the root only
/// forwards it, while sub-components additionally alias + lower its function.
fn import_pca_instance(
    types: &mut wasm_encoder::ComponentTypeSection,
    imports: &mut wasm_encoder::ComponentImportSection,
) {
    use wasm_encoder::{ComponentTypeRef, ComponentValType, InstanceType, PrimitiveValType};

    let u32_ty = ComponentValType::Primitive(PrimitiveValType::U32);
    let mut pca_ty = InstanceType::new();
    pca_ty
        .ty()
        .function()
        .params([("category", u32_ty), ("count", u32_ty)])
        .result(None);
    pca_ty.export(INCREMENT_FN, ComponentTypeRef::Func(0));
    types.instance(&pca_ty);
    imports.import(PCA_INSTANCE, ComponentTypeRef::Instance(0));
}

/// Instrument a component for the
/// dynamic instruction mix.
///
/// Every component (the root or, recursively, any sub-component) is transformed
/// the same way: it imports a `sightglass-pca` instance and threads
/// `increment-instruction-count` down into everything that needs it.
///
/// We prepend, before the component's original content, five items that take
/// the lowest index in each space (so every original reference shifts up by
/// one, applied only at depth 0; nested type scopes keep their own index
/// spaces):
///
/// * Component type 0: the `sightglass-pca` instance type
/// * Component instance 0: an import of the `sightglass-pca` instance
/// * Component func 0: an alias of `sightglass-pca.increment-instruction-count`
/// * Core func 0: a `canon lower` of that function
/// * Core instance 0: a core instance of just that lowered function
///
/// A component with no core modules of its own (e.g. a root component that just
/// instantiates and links subcomponents) simply never uses the lowered function
/// or core instance.
pub(super) fn instrument_component(wasm: &[u8]) -> Result<Vec<u8>> {
    use wasm_encoder::reencode::{
        component_utils, Error as ReencodeError, Reencode, ReencodeComponent,
    };
    use wasm_encoder::{
        Alias, CanonicalFunctionSection, Component, ComponentAliasSection, ComponentExportKind,
        ComponentImportSection, ComponentInstanceSection, ComponentSectionId, ComponentTypeSection,
        ExportKind, InstanceSection, ModuleArg, ModuleSection, RawSection,
    };

    struct Instrumenter {
        depth: u32,
        prepended: bool,
    }

    impl Instrumenter {
        /// Shift an index up by one, but only in this component's own (depth-0)
        /// index spaces, not inside nested type declarations.
        fn shift(&self, i: u32) -> u32 {
            if self.depth == 0 {
                i + 1
            } else {
                i
            }
        }

        fn emit_prelude(component: &mut Component) {
            let mut types = ComponentTypeSection::new();
            let mut imports = ComponentImportSection::new();
            import_pca_instance(&mut types, &mut imports);
            component.section(&types);
            component.section(&imports);

            // Alias `increment-instruction-count` out of the instance
            // (component func 0), lower it (core func 0), and wrap it in a core
            // instance (core instance 0) for the core modules to import.
            let mut aliases = ComponentAliasSection::new();
            aliases.alias(Alias::InstanceExport {
                instance: 0,
                kind: ComponentExportKind::Func,
                name: INCREMENT_FN,
            });
            component.section(&aliases);

            let mut canon = CanonicalFunctionSection::new();
            canon.lower(0, no_opts());
            component.section(&canon);

            let mut instances = InstanceSection::new();
            instances.export_items([(INCREMENT_FN, ExportKind::Func, 0u32)]);
            component.section(&instances);
        }
    }

    impl Reencode for Instrumenter {
        type Error = anyhow::Error;
        fn function_index(&mut self, i: u32) -> Result<u32, ReencodeError<Self::Error>> {
            Ok(self.shift(i))
        }
    }

    impl ReencodeComponent for Instrumenter {
        fn component_func_index(&mut self, i: u32) -> u32 {
            self.shift(i)
        }
        fn component_type_index(&mut self, i: u32) -> u32 {
            self.shift(i)
        }
        fn component_instance_index(&mut self, i: u32) -> u32 {
            self.shift(i)
        }
        fn instance_index(&mut self, i: u32) -> u32 {
            self.shift(i)
        }
        fn outer_component_type_index(&mut self, count: u32, i: u32) -> u32 {
            // An `alias outer (type ...)` reaching up to this component's own
            // (depth-0) component type space must account for the prepended
            // type.
            if count == self.depth {
                i + 1
            } else {
                i
            }
        }
        fn push_depth(&mut self) {
            self.depth += 1;
        }
        fn pop_depth(&mut self) {
            self.depth -= 1;
        }

        fn parse_component_payload(
            &mut self,
            component: &mut Component,
            payload: wasmparser::Payload<'_>,
            whole: &[u8],
        ) -> Result<(), ReencodeError<Self::Error>> {
            // Prepend the prelude after any leading sub-components / core modules,
            // so it lands in the import region (and is component instance 0).
            if !self.prepended
                && !matches!(
                    payload,
                    wasmparser::Payload::Version { .. }
                        | wasmparser::Payload::ComponentSection { .. }
                        | wasmparser::Payload::ModuleSection { .. }
                )
            {
                self.prepended = true;
                Self::emit_prelude(component);
            }
            component_utils::parse_component_payload(self, component, payload, whole)
        }

        fn parse_component_subcomponent(
            &mut self,
            component: &mut Component,
            _parser: wasmparser::Parser,
            subcomponent: &[u8],
            _whole: &[u8],
        ) -> Result<(), ReencodeError<Self::Error>> {
            // A nested sub-component is instrumented exactly like the root.
            let instrumented =
                instrument_component(subcomponent).map_err(ReencodeError::UserError)?;
            component.section(&RawSection {
                id: ComponentSectionId::Component.into(),
                data: &instrumented,
            });
            Ok(())
        }

        fn parse_component_instance(
            &mut self,
            instances: &mut ComponentInstanceSection,
            instance: wasmparser::ComponentInstance<'_>,
        ) -> Result<(), ReencodeError<Self::Error>> {
            match instance {
                wasmparser::ComponentInstance::Instantiate {
                    component_index,
                    args,
                } => {
                    let mut new_args: Vec<(&str, ComponentExportKind, u32)> = args
                        .iter()
                        .map(|arg| {
                            (
                                arg.name,
                                arg.kind.into(),
                                self.component_external_index(arg.kind, arg.index),
                            )
                        })
                        .collect();
                    // Pass the `sightglass-pca` *instance* (component instance 0)
                    // to each instantiated sub-component.
                    new_args.push((PCA_INSTANCE, ComponentExportKind::Instance, 0));
                    instances.instantiate(self.component_index(component_index), new_args);
                }
                wasmparser::ComponentInstance::FromExports(exports) => {
                    let items: Vec<(&str, ComponentExportKind, u32)> = exports
                        .iter()
                        .map(|e| {
                            (
                                e.name.0,
                                e.kind.into(),
                                self.component_external_index(e.kind, e.index),
                            )
                        })
                        .collect();
                    instances.export_items(items);
                }
            }
            Ok(())
        }

        fn parse_component_submodule(
            &mut self,
            component: &mut Component,
            _parser: wasmparser::Parser,
            module: &[u8],
        ) -> Result<(), ReencodeError<Self::Error>> {
            // Reencode the core module to count and flush its instructions.
            let instrumented =
                super::instrument_core_module(module, true).map_err(ReencodeError::UserError)?;
            component.section(&ModuleSection(&instrumented));
            Ok(())
        }

        fn parse_instance(
            &mut self,
            instances: &mut InstanceSection,
            instance: wasmparser::Instance<'_>,
        ) -> Result<(), ReencodeError<Self::Error>> {
            match instance {
                wasmparser::Instance::Instantiate { module_index, args } => {
                    let mut new_args: Vec<(&str, ModuleArg)> = args
                        .iter()
                        .map(|arg| match arg.kind {
                            wasmparser::InstantiationArgKind::Instance => (
                                arg.name,
                                ModuleArg::Instance(self.instance_index(arg.index)),
                            ),
                        })
                        .collect();
                    // Supply the instrumented module's `sightglass-pca` import
                    // from the prepended core instance (index 0).
                    new_args.push((PCA_INSTANCE, ModuleArg::Instance(0)));
                    instances.instantiate(self.module_index(module_index), new_args);
                }
                wasmparser::Instance::FromExports(exports) => {
                    let exports = exports
                        .iter()
                        .map(|export| {
                            Ok((
                                export.name,
                                self.export_kind(export.kind)?,
                                self.external_index(export.kind, export.index)?,
                            ))
                        })
                        .collect::<Result<Vec<_>, ReencodeError<Self::Error>>>()?;
                    instances.export_items(exports);
                }
            }
            Ok(())
        }
    }

    let mut component = Component::new();
    let mut instrumenter = Instrumenter {
        depth: 0,
        prepended: false,
    };
    instrumenter
        .parse_component(&mut component, wasmparser::Parser::new(0), wasm)
        .map_err(|e| anyhow::anyhow!("failed to reencode component: {e}"))?;
    Ok(component.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Instrumenting a core module keeps it valid and behaving the same
    /// (proving the function-index shift is applied everywhere), accumulates
    /// each block's operators into the counter globals, and flushes them to the
    /// imported `increment-instruction-count` at the next function entry.
    #[test]
    fn instrumented_core_module_flushes_counts() -> anyhow::Result<()> {
        use wasm_encoder::{
            CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, Module,
            TypeSection, ValType,
        };

        // Two functions: `work() -> i32` = `1 + 2` (four operators), and an
        // empty `flush()`. Running `work` accumulates its counts into the
        // globals; calling `flush` afterward pushes them to the host at
        // `flush`'s entry.
        let mut types = TypeSection::new();
        types.ty().function([], [ValType::I32]); // 0: work
        types.ty().function([], []); // 1: flush
        let mut funcs = FunctionSection::new();
        funcs.function(0);
        funcs.function(1);
        let mut exports = ExportSection::new();
        exports.export("work", ExportKind::Func, 0);
        exports.export("flush", ExportKind::Func, 1);
        let mut code = CodeSection::new();
        let mut work = Function::new([]);
        work.instruction(&Instruction::I32Const(1));
        work.instruction(&Instruction::I32Const(2));
        work.instruction(&Instruction::I32Add);
        work.instruction(&Instruction::End);
        code.function(&work);
        let mut flush = Function::new([]);
        flush.instruction(&Instruction::End);
        code.function(&flush);
        let mut m = Module::new();
        m.section(&types);
        m.section(&funcs);
        m.section(&exports);
        m.section(&code);
        let input = m.finish();

        let instrumented = super::super::instrument_core_module(&input, true)?.finish();

        let engine = wasmtime::Engine::default();
        let module = wasmtime::Module::new(&engine, &instrumented)?;
        let mut store = wasmtime::Store::new(&engine, 0u64);
        let mut linker = wasmtime::Linker::new(&engine);
        linker.func_wrap(
            super::super::PCA_INSTANCE,
            super::super::INCREMENT_FN,
            |mut caller: wasmtime::Caller<'_, u64>, _category: u32, count: u32| {
                *caller.data_mut() += u64::from(count);
            },
        )?;
        let instance = linker.instantiate(&mut store, &module)?;
        let work = instance.get_typed_func::<(), i32>(&mut store, "work")?;
        let flush = instance.get_typed_func::<(), ()>(&mut store, "flush")?;

        // `work` still computes 1 + 2, and its four operators are flushed to
        // the host when `flush`'s entry pushes the accumulated counters.
        assert_eq!(work.call(&mut store, ())?, 3);
        flush.call(&mut store, ())?;
        assert_eq!(
            *store.data(),
            4,
            "work's four operators flushed to the host"
        );

        Ok(())
    }

    /// Instrumenting the real `cm-online-stats` component must produce a
    /// component that still validates. This exercises the whole transform
    /// without running the (multi-million iteration) benchmark.
    #[test]
    fn instrumented_component_validates() -> anyhow::Result<()> {
        let path = "../../benchmarks/cm-online-stats/cm-online-stats.wasm";
        let wasm = match std::fs::read(path) {
            Ok(wasm) => wasm,
            // The benchmark isn't present in this checkout; nothing to test.
            Err(_) => return Ok(()),
        };

        let instrumented = instrument_component(&wasm)?;
        let engine = super::super::make_engine(false)?;
        wasmtime::component::Component::new(&engine, &instrumented)?;
        Ok(())
    }
}
