//! Categorization of Wasm operators into the instruction categories whose static
//! and dynamic mixes we track for PCA.

use anyhow::{bail, Result};
use wasmparser::Operator;

/// The categories of Wasm instructions whose static and dynamic mixes we track.
///
/// The discriminants double as indices into the per-category counter arrays (see
/// the `Counts` accumulator) and as the offsets of the instrumentation's counter
/// globals (see the `dynamic_metrics` module), so their order must stay in sync
/// with both.
#[derive(Clone, Copy)]
pub(crate) enum Category {
    Unreachable = 0,
    Nop = 1,
    ControlBranch = 2,
    ControlCall = 3,
    ControlException = 4,
    ControlStackSwitch = 5,
    LocalVariable = 6,
    GlobalVariable = 7,
    AtomicGlobalVariable = 8,
    Table = 9,
    AtomicTable = 10,
    MemorySize = 11,
    MemoryGrow = 12,
    MemoryLoad = 13,
    MemoryStore = 14,
    MemoryOther = 15,
    Ref = 16,
    I31 = 17,
    AggregateNew = 18,
    AggregateGet = 19,
    AggregateSet = 20,
    AtomicAggregate = 21,
    NumericInteger = 22,
    NumericFloat = 23,
    Vector = 24,
    Select = 25,
}

/// The total number of [`Category`] variants.
pub(crate) const NUM_CATEGORIES: usize = 26;

impl Category {
    /// Classify a Wasm operator into its [`Category`], returning an error for
    /// any operator we don't recognize.
    pub(crate) fn for_op(op: &Operator) -> Result<Category> {
        Ok(match op {
            Operator::Unreachable => Category::Unreachable,
            Operator::Drop | Operator::Nop => Category::Nop,
            Operator::Block { .. }
            | Operator::Br { .. }
            | Operator::BrIf { .. }
            | Operator::BrOnNonNull { .. }
            | Operator::BrOnNull { .. }
            | Operator::BrTable { .. }
            | Operator::Else
            | Operator::End
            | Operator::If { .. }
            | Operator::Loop { .. }
            | Operator::Return => Category::ControlBranch,
            Operator::Call { .. }
            | Operator::CallIndirect { .. }
            | Operator::CallRef { .. }
            | Operator::ReturnCall { .. }
            | Operator::ReturnCallIndirect { .. }
            | Operator::ReturnCallRef { .. } => Category::ControlCall,
            Operator::Catch { .. }
            | Operator::CatchAll
            | Operator::Delegate { .. }
            | Operator::Rethrow { .. }
            | Operator::Throw { .. }
            | Operator::ThrowRef
            | Operator::Try { .. }
            | Operator::TryTable { .. } => Category::ControlException,
            Operator::ContBind { .. }
            | Operator::ContNew { .. }
            | Operator::Resume { .. }
            | Operator::ResumeThrow { .. }
            | Operator::Suspend { .. }
            | Operator::Switch { .. } => Category::ControlStackSwitch,
            Operator::LocalGet { .. } | Operator::LocalSet { .. } | Operator::LocalTee { .. } => {
                Category::LocalVariable
            }
            Operator::GlobalGet { .. } | Operator::GlobalSet { .. } => Category::GlobalVariable,
            Operator::GlobalAtomicGet { .. }
            | Operator::GlobalAtomicRmwAdd { .. }
            | Operator::GlobalAtomicRmwAnd { .. }
            | Operator::GlobalAtomicRmwCmpxchg { .. }
            | Operator::GlobalAtomicRmwOr { .. }
            | Operator::GlobalAtomicRmwSub { .. }
            | Operator::GlobalAtomicRmwXchg { .. }
            | Operator::GlobalAtomicRmwXor { .. }
            | Operator::GlobalAtomicSet { .. } => Category::AtomicGlobalVariable,
            Operator::ElemDrop { .. }
            | Operator::TableCopy { .. }
            | Operator::TableFill { .. }
            | Operator::TableGet { .. }
            | Operator::TableGrow { .. }
            | Operator::TableInit { .. }
            | Operator::TableSet { .. }
            | Operator::TableSize { .. } => Category::Table,
            Operator::TableAtomicGet { .. }
            | Operator::TableAtomicRmwCmpxchg { .. }
            | Operator::TableAtomicRmwXchg { .. }
            | Operator::TableAtomicSet { .. } => Category::AtomicTable,
            Operator::MemorySize { .. } => Category::MemorySize,
            Operator::MemoryGrow { .. } => Category::MemoryGrow,
            Operator::F32Load { .. }
            | Operator::F64Load { .. }
            | Operator::I32AtomicLoad { .. }
            | Operator::I32AtomicLoad16U { .. }
            | Operator::I32AtomicLoad8U { .. }
            | Operator::I32Load { .. }
            | Operator::I32Load16S { .. }
            | Operator::I32Load16U { .. }
            | Operator::I32Load8S { .. }
            | Operator::I32Load8U { .. }
            | Operator::I64AtomicLoad { .. }
            | Operator::I64AtomicLoad16U { .. }
            | Operator::I64AtomicLoad32U { .. }
            | Operator::I64AtomicLoad8U { .. }
            | Operator::I64Load { .. }
            | Operator::I64Load16S { .. }
            | Operator::I64Load16U { .. }
            | Operator::I64Load32S { .. }
            | Operator::I64Load32U { .. }
            | Operator::I64Load8S { .. }
            | Operator::I64Load8U { .. }
            | Operator::V128Load { .. }
            | Operator::V128Load16Lane { .. }
            | Operator::V128Load16Splat { .. }
            | Operator::V128Load16x4S { .. }
            | Operator::V128Load16x4U { .. }
            | Operator::V128Load32Lane { .. }
            | Operator::V128Load32Splat { .. }
            | Operator::V128Load32Zero { .. }
            | Operator::V128Load32x2S { .. }
            | Operator::V128Load32x2U { .. }
            | Operator::V128Load64Lane { .. }
            | Operator::V128Load64Splat { .. }
            | Operator::V128Load64Zero { .. }
            | Operator::V128Load8Lane { .. }
            | Operator::V128Load8Splat { .. }
            | Operator::V128Load8x8S { .. }
            | Operator::V128Load8x8U { .. } => Category::MemoryLoad,
            Operator::F32Store { .. }
            | Operator::F64Store { .. }
            | Operator::I32AtomicStore { .. }
            | Operator::I32AtomicStore16 { .. }
            | Operator::I32AtomicStore8 { .. }
            | Operator::I32Store { .. }
            | Operator::I32Store16 { .. }
            | Operator::I32Store8 { .. }
            | Operator::I64AtomicStore { .. }
            | Operator::I64AtomicStore16 { .. }
            | Operator::I64AtomicStore32 { .. }
            | Operator::I64AtomicStore8 { .. }
            | Operator::I64Store { .. }
            | Operator::I64Store16 { .. }
            | Operator::I64Store32 { .. }
            | Operator::I64Store8 { .. }
            | Operator::V128Store { .. }
            | Operator::V128Store16Lane { .. }
            | Operator::V128Store32Lane { .. }
            | Operator::V128Store64Lane { .. }
            | Operator::V128Store8Lane { .. } => Category::MemoryStore,
            Operator::AtomicFence
            | Operator::DataDrop { .. }
            | Operator::I32AtomicRmw16AddU { .. }
            | Operator::I32AtomicRmw16AndU { .. }
            | Operator::I32AtomicRmw16CmpxchgU { .. }
            | Operator::I32AtomicRmw16OrU { .. }
            | Operator::I32AtomicRmw16SubU { .. }
            | Operator::I32AtomicRmw16XchgU { .. }
            | Operator::I32AtomicRmw16XorU { .. }
            | Operator::I32AtomicRmw8AddU { .. }
            | Operator::I32AtomicRmw8AndU { .. }
            | Operator::I32AtomicRmw8CmpxchgU { .. }
            | Operator::I32AtomicRmw8OrU { .. }
            | Operator::I32AtomicRmw8SubU { .. }
            | Operator::I32AtomicRmw8XchgU { .. }
            | Operator::I32AtomicRmw8XorU { .. }
            | Operator::I32AtomicRmwAdd { .. }
            | Operator::I32AtomicRmwAnd { .. }
            | Operator::I32AtomicRmwCmpxchg { .. }
            | Operator::I32AtomicRmwOr { .. }
            | Operator::I32AtomicRmwSub { .. }
            | Operator::I32AtomicRmwXchg { .. }
            | Operator::I32AtomicRmwXor { .. }
            | Operator::I64AtomicRmw16AddU { .. }
            | Operator::I64AtomicRmw16AndU { .. }
            | Operator::I64AtomicRmw16CmpxchgU { .. }
            | Operator::I64AtomicRmw16OrU { .. }
            | Operator::I64AtomicRmw16SubU { .. }
            | Operator::I64AtomicRmw16XchgU { .. }
            | Operator::I64AtomicRmw16XorU { .. }
            | Operator::I64AtomicRmw32AddU { .. }
            | Operator::I64AtomicRmw32AndU { .. }
            | Operator::I64AtomicRmw32CmpxchgU { .. }
            | Operator::I64AtomicRmw32OrU { .. }
            | Operator::I64AtomicRmw32SubU { .. }
            | Operator::I64AtomicRmw32XchgU { .. }
            | Operator::I64AtomicRmw32XorU { .. }
            | Operator::I64AtomicRmw8AddU { .. }
            | Operator::I64AtomicRmw8AndU { .. }
            | Operator::I64AtomicRmw8CmpxchgU { .. }
            | Operator::I64AtomicRmw8OrU { .. }
            | Operator::I64AtomicRmw8SubU { .. }
            | Operator::I64AtomicRmw8XchgU { .. }
            | Operator::I64AtomicRmw8XorU { .. }
            | Operator::I64AtomicRmwAdd { .. }
            | Operator::I64AtomicRmwAnd { .. }
            | Operator::I64AtomicRmwCmpxchg { .. }
            | Operator::I64AtomicRmwOr { .. }
            | Operator::I64AtomicRmwSub { .. }
            | Operator::I64AtomicRmwXchg { .. }
            | Operator::I64AtomicRmwXor { .. }
            | Operator::MemoryAtomicNotify { .. }
            | Operator::MemoryAtomicWait32 { .. }
            | Operator::MemoryAtomicWait64 { .. }
            | Operator::MemoryCopy { .. }
            | Operator::MemoryDiscard { .. }
            | Operator::MemoryFill { .. }
            | Operator::MemoryInit { .. } => Category::MemoryOther,
            Operator::ExternConvertAny
            | Operator::RefAsNonNull
            | Operator::RefCastDescNonNull { .. }
            | Operator::RefCastDescNullable { .. }
            | Operator::RefCastNonNull { .. }
            | Operator::RefCastNullable { .. }
            | Operator::RefEq
            | Operator::RefFunc { .. }
            | Operator::RefGetDesc { .. }
            | Operator::RefIsNull
            | Operator::RefNull { .. }
            | Operator::RefTestNonNull { .. }
            | Operator::RefTestNullable { .. } => Category::Ref,
            Operator::I31GetS | Operator::I31GetU | Operator::RefI31 | Operator::RefI31Shared => {
                Category::I31
            }
            Operator::ArrayNew { .. }
            | Operator::ArrayNewData { .. }
            | Operator::ArrayNewDefault { .. }
            | Operator::ArrayNewElem { .. }
            | Operator::ArrayNewFixed { .. }
            | Operator::StructNew { .. }
            | Operator::StructNewDefault { .. }
            | Operator::StructNewDefaultDesc { .. }
            | Operator::StructNewDesc { .. } => Category::AggregateNew,
            Operator::ArrayGet { .. }
            | Operator::ArrayGetS { .. }
            | Operator::ArrayGetU { .. }
            | Operator::ArrayLen
            | Operator::StructGet { .. }
            | Operator::StructGetS { .. }
            | Operator::StructGetU { .. } => Category::AggregateGet,
            Operator::ArrayCopy { .. }
            | Operator::ArrayFill { .. }
            | Operator::ArrayInitData { .. }
            | Operator::ArrayInitElem { .. }
            | Operator::ArraySet { .. }
            | Operator::StructSet { .. } => Category::AggregateSet,
            Operator::ArrayAtomicGet { .. }
            | Operator::ArrayAtomicGetS { .. }
            | Operator::ArrayAtomicGetU { .. }
            | Operator::ArrayAtomicRmwAdd { .. }
            | Operator::ArrayAtomicRmwAnd { .. }
            | Operator::ArrayAtomicRmwCmpxchg { .. }
            | Operator::ArrayAtomicRmwOr { .. }
            | Operator::ArrayAtomicRmwSub { .. }
            | Operator::ArrayAtomicRmwXchg { .. }
            | Operator::ArrayAtomicRmwXor { .. }
            | Operator::ArrayAtomicSet { .. }
            | Operator::StructAtomicGet { .. }
            | Operator::StructAtomicGetS { .. }
            | Operator::StructAtomicGetU { .. }
            | Operator::StructAtomicRmwAdd { .. }
            | Operator::StructAtomicRmwAnd { .. }
            | Operator::StructAtomicRmwCmpxchg { .. }
            | Operator::StructAtomicRmwOr { .. }
            | Operator::StructAtomicRmwSub { .. }
            | Operator::StructAtomicRmwXchg { .. }
            | Operator::StructAtomicRmwXor { .. }
            | Operator::StructAtomicSet { .. } => Category::AtomicAggregate,
            Operator::I32Add
            | Operator::I32And
            | Operator::I32Clz
            | Operator::I32Const { .. }
            | Operator::I32Ctz
            | Operator::I32DivS
            | Operator::I32DivU
            | Operator::I32Eq
            | Operator::I32Eqz
            | Operator::I32Extend16S
            | Operator::I32Extend8S
            | Operator::I32GeS
            | Operator::I32GeU
            | Operator::I32GtS
            | Operator::I32GtU
            | Operator::I32LeS
            | Operator::I32LeU
            | Operator::I32LtS
            | Operator::I32LtU
            | Operator::I32Mul
            | Operator::I32Ne
            | Operator::I32Or
            | Operator::I32Popcnt
            | Operator::I32ReinterpretF32
            | Operator::I32RemS
            | Operator::I32RemU
            | Operator::I32Rotl
            | Operator::I32Rotr
            | Operator::I32Shl
            | Operator::I32ShrS
            | Operator::I32ShrU
            | Operator::I32Sub
            | Operator::I32TruncF32S
            | Operator::I32TruncF32U
            | Operator::I32TruncF64S
            | Operator::I32TruncF64U
            | Operator::I32TruncSatF32S
            | Operator::I32TruncSatF32U
            | Operator::I32TruncSatF64S
            | Operator::I32TruncSatF64U
            | Operator::I32WrapI64
            | Operator::I32Xor
            | Operator::I64Add
            | Operator::I64Add128
            | Operator::I64And
            | Operator::I64Clz
            | Operator::I64Const { .. }
            | Operator::I64Ctz
            | Operator::I64DivS
            | Operator::I64DivU
            | Operator::I64Eq
            | Operator::I64Eqz
            | Operator::I64Extend16S
            | Operator::I64Extend32S
            | Operator::I64Extend8S
            | Operator::I64ExtendI32S
            | Operator::I64ExtendI32U
            | Operator::I64GeS
            | Operator::I64GeU
            | Operator::I64GtS
            | Operator::I64GtU
            | Operator::I64LeS
            | Operator::I64LeU
            | Operator::I64LtS
            | Operator::I64LtU
            | Operator::I64Mul
            | Operator::I64MulWideS
            | Operator::I64MulWideU
            | Operator::I64Ne
            | Operator::I64Or
            | Operator::I64Popcnt
            | Operator::I64ReinterpretF64
            | Operator::I64RemS
            | Operator::I64RemU
            | Operator::I64Rotl
            | Operator::I64Rotr
            | Operator::I64Shl
            | Operator::I64ShrS
            | Operator::I64ShrU
            | Operator::I64Sub
            | Operator::I64Sub128
            | Operator::I64TruncF32S
            | Operator::I64TruncF32U
            | Operator::I64TruncF64S
            | Operator::I64TruncF64U
            | Operator::I64TruncSatF32S
            | Operator::I64TruncSatF32U
            | Operator::I64TruncSatF64S
            | Operator::I64TruncSatF64U
            | Operator::I64Xor => Category::NumericInteger,
            Operator::F32Abs
            | Operator::F32Add
            | Operator::F32Ceil
            | Operator::F32Const { .. }
            | Operator::F32ConvertI32S
            | Operator::F32ConvertI32U
            | Operator::F32ConvertI64S
            | Operator::F32ConvertI64U
            | Operator::F32Copysign
            | Operator::F32DemoteF64
            | Operator::F32Div
            | Operator::F32Eq
            | Operator::F32Floor
            | Operator::F32Ge
            | Operator::F32Gt
            | Operator::F32Le
            | Operator::F32Lt
            | Operator::F32Max
            | Operator::F32Min
            | Operator::F32Mul
            | Operator::F32Ne
            | Operator::F32Nearest
            | Operator::F32Neg
            | Operator::F32ReinterpretI32
            | Operator::F32Sqrt
            | Operator::F32Sub
            | Operator::F32Trunc
            | Operator::F64Abs
            | Operator::F64Add
            | Operator::F64Ceil
            | Operator::F64Const { .. }
            | Operator::F64ConvertI32S
            | Operator::F64ConvertI32U
            | Operator::F64ConvertI64S
            | Operator::F64ConvertI64U
            | Operator::F64Copysign
            | Operator::F64Div
            | Operator::F64Eq
            | Operator::F64Floor
            | Operator::F64Ge
            | Operator::F64Gt
            | Operator::F64Le
            | Operator::F64Lt
            | Operator::F64Max
            | Operator::F64Min
            | Operator::F64Mul
            | Operator::F64Ne
            | Operator::F64Nearest
            | Operator::F64Neg
            | Operator::F64PromoteF32
            | Operator::F64ReinterpretI64
            | Operator::F64Sqrt
            | Operator::F64Sub
            | Operator::F64Trunc => Category::NumericFloat,
            Operator::F32x4Abs
            | Operator::F32x4Add
            | Operator::F32x4Ceil
            | Operator::F32x4ConvertI32x4S
            | Operator::F32x4ConvertI32x4U
            | Operator::F32x4DemoteF64x2Zero
            | Operator::F32x4Div
            | Operator::F32x4Eq
            | Operator::F32x4ExtractLane { .. }
            | Operator::F32x4Floor
            | Operator::F32x4Ge
            | Operator::F32x4Gt
            | Operator::F32x4Le
            | Operator::F32x4Lt
            | Operator::F32x4Max
            | Operator::F32x4Min
            | Operator::F32x4Mul
            | Operator::F32x4Ne
            | Operator::F32x4Nearest
            | Operator::F32x4Neg
            | Operator::F32x4PMax
            | Operator::F32x4PMin
            | Operator::F32x4RelaxedMadd
            | Operator::F32x4RelaxedMax
            | Operator::F32x4RelaxedMin
            | Operator::F32x4RelaxedNmadd
            | Operator::F32x4ReplaceLane { .. }
            | Operator::F32x4Splat
            | Operator::F32x4Sqrt
            | Operator::F32x4Sub
            | Operator::F32x4Trunc
            | Operator::F64x2Abs
            | Operator::F64x2Add
            | Operator::F64x2Ceil
            | Operator::F64x2ConvertLowI32x4S
            | Operator::F64x2ConvertLowI32x4U
            | Operator::F64x2Div
            | Operator::F64x2Eq
            | Operator::F64x2ExtractLane { .. }
            | Operator::F64x2Floor
            | Operator::F64x2Ge
            | Operator::F64x2Gt
            | Operator::F64x2Le
            | Operator::F64x2Lt
            | Operator::F64x2Max
            | Operator::F64x2Min
            | Operator::F64x2Mul
            | Operator::F64x2Ne
            | Operator::F64x2Nearest
            | Operator::F64x2Neg
            | Operator::F64x2PMax
            | Operator::F64x2PMin
            | Operator::F64x2PromoteLowF32x4
            | Operator::F64x2RelaxedMax
            | Operator::F64x2RelaxedMin
            | Operator::F64x2RelaxedNmadd
            | Operator::F64x2ReplaceLane { .. }
            | Operator::F64x2Splat
            | Operator::F64x2Sqrt
            | Operator::F64x2Sub
            | Operator::F64x2Trunc
            | Operator::I16x8Abs
            | Operator::I16x8Add
            | Operator::I16x8AddSatS
            | Operator::I16x8AddSatU
            | Operator::I16x8AllTrue
            | Operator::I16x8AvgrU
            | Operator::I16x8Bitmask
            | Operator::I16x8Eq
            | Operator::I16x8ExtAddPairwiseI8x16S
            | Operator::I16x8ExtAddPairwiseI8x16U
            | Operator::I16x8ExtMulHighI8x16S
            | Operator::I16x8ExtMulHighI8x16U
            | Operator::I16x8ExtMulLowI8x16S
            | Operator::I16x8ExtMulLowI8x16U
            | Operator::I16x8ExtendHighI8x16S
            | Operator::I16x8ExtendHighI8x16U
            | Operator::I16x8ExtendLowI8x16S
            | Operator::I16x8ExtendLowI8x16U
            | Operator::I16x8ExtractLaneS { .. }
            | Operator::I16x8ExtractLaneU { .. }
            | Operator::I16x8GeS
            | Operator::I16x8GeU
            | Operator::I16x8GtS
            | Operator::I16x8GtU
            | Operator::I16x8LeS
            | Operator::I16x8LeU
            | Operator::I16x8LtS
            | Operator::I16x8LtU
            | Operator::I16x8MaxS
            | Operator::I16x8MaxU
            | Operator::I16x8MinS
            | Operator::I16x8MinU
            | Operator::I16x8Mul
            | Operator::I16x8NarrowI32x4S
            | Operator::I16x8NarrowI32x4U
            | Operator::I16x8Ne
            | Operator::I16x8Neg
            | Operator::I16x8Q15MulrSatS
            | Operator::I16x8RelaxedDotI8x16I7x16S
            | Operator::I16x8RelaxedLaneselect
            | Operator::I16x8RelaxedQ15mulrS
            | Operator::I16x8ReplaceLane { .. }
            | Operator::I16x8Shl
            | Operator::I16x8ShrS
            | Operator::I16x8ShrU
            | Operator::I16x8Splat
            | Operator::I16x8Sub
            | Operator::I16x8SubSatS
            | Operator::I16x8SubSatU
            | Operator::I32x4Abs
            | Operator::I32x4Add
            | Operator::I32x4AllTrue
            | Operator::I32x4Bitmask
            | Operator::I32x4DotI16x8S
            | Operator::I32x4Eq
            | Operator::I32x4ExtAddPairwiseI16x8S
            | Operator::I32x4ExtAddPairwiseI16x8U
            | Operator::I32x4ExtMulHighI16x8S
            | Operator::I32x4ExtMulHighI16x8U
            | Operator::I32x4ExtMulLowI16x8S
            | Operator::I32x4ExtMulLowI16x8U
            | Operator::I32x4ExtendHighI16x8S
            | Operator::I32x4ExtendHighI16x8U
            | Operator::I32x4ExtendLowI16x8S
            | Operator::I32x4ExtendLowI16x8U
            | Operator::I32x4ExtractLane { .. }
            | Operator::I32x4GeS
            | Operator::I32x4GeU
            | Operator::I32x4GtS
            | Operator::I32x4GtU
            | Operator::I32x4LeS
            | Operator::I32x4LeU
            | Operator::I32x4LtS
            | Operator::I32x4LtU
            | Operator::I32x4MaxS
            | Operator::I32x4MaxU
            | Operator::I32x4MinS
            | Operator::I32x4MinU
            | Operator::I32x4Mul
            | Operator::I32x4Ne
            | Operator::I32x4Neg
            | Operator::I32x4RelaxedDotI8x16I7x16AddS
            | Operator::I32x4RelaxedLaneselect
            | Operator::I32x4RelaxedTruncF32x4S
            | Operator::I32x4RelaxedTruncF32x4U
            | Operator::I32x4RelaxedTruncF64x2SZero
            | Operator::I32x4RelaxedTruncF64x2UZero
            | Operator::I32x4ReplaceLane { .. }
            | Operator::I32x4Shl
            | Operator::I32x4ShrS
            | Operator::I32x4ShrU
            | Operator::I32x4Splat
            | Operator::I32x4Sub
            | Operator::I32x4TruncSatF32x4S
            | Operator::I32x4TruncSatF32x4U
            | Operator::I32x4TruncSatF64x2SZero
            | Operator::I32x4TruncSatF64x2UZero
            | Operator::I64x2Abs
            | Operator::I64x2Add
            | Operator::I64x2AllTrue
            | Operator::I64x2Bitmask
            | Operator::I64x2Eq
            | Operator::I64x2ExtMulHighI32x4S
            | Operator::I64x2ExtMulHighI32x4U
            | Operator::I64x2ExtMulLowI32x4S
            | Operator::I64x2ExtMulLowI32x4U
            | Operator::I64x2ExtendHighI32x4S
            | Operator::I64x2ExtendHighI32x4U
            | Operator::I64x2ExtendLowI32x4S
            | Operator::I64x2ExtendLowI32x4U
            | Operator::I64x2ExtractLane { .. }
            | Operator::I64x2GeS
            | Operator::I64x2GtS
            | Operator::I64x2LeS
            | Operator::I64x2LtS
            | Operator::I64x2Mul
            | Operator::I64x2Ne
            | Operator::I64x2Neg
            | Operator::I64x2RelaxedLaneselect
            | Operator::I64x2ReplaceLane { .. }
            | Operator::I64x2Shl
            | Operator::I64x2ShrS
            | Operator::I64x2ShrU
            | Operator::I64x2Splat
            | Operator::I64x2Sub
            | Operator::I8x16Abs
            | Operator::I8x16Add
            | Operator::I8x16AddSatS
            | Operator::I8x16AddSatU
            | Operator::I8x16AllTrue
            | Operator::I8x16AvgrU
            | Operator::I8x16Bitmask
            | Operator::I8x16Eq
            | Operator::I8x16ExtractLaneS { .. }
            | Operator::I8x16ExtractLaneU { .. }
            | Operator::I8x16GeS
            | Operator::I8x16GeU
            | Operator::I8x16GtS
            | Operator::I8x16GtU
            | Operator::I8x16LeS
            | Operator::I8x16LeU
            | Operator::I8x16LtS
            | Operator::I8x16LtU
            | Operator::I8x16MaxS
            | Operator::I8x16MaxU
            | Operator::I8x16MinS
            | Operator::I8x16MinU
            | Operator::I8x16NarrowI16x8S
            | Operator::I8x16NarrowI16x8U
            | Operator::I8x16Ne
            | Operator::I8x16Neg
            | Operator::I8x16Popcnt
            | Operator::I8x16RelaxedLaneselect
            | Operator::I8x16RelaxedSwizzle
            | Operator::I8x16ReplaceLane { .. }
            | Operator::I8x16Shl
            | Operator::I8x16ShrS
            | Operator::I8x16ShrU
            | Operator::I8x16Shuffle { .. }
            | Operator::I8x16Splat
            | Operator::I8x16Sub
            | Operator::I8x16SubSatS
            | Operator::I8x16SubSatU
            | Operator::I8x16Swizzle
            | Operator::V128And
            | Operator::V128AndNot
            | Operator::V128AnyTrue
            | Operator::V128Bitselect
            | Operator::V128Const { .. }
            | Operator::V128Not
            | Operator::V128Or
            | Operator::V128Xor => Category::Vector,
            Operator::Select | Operator::TypedSelect { .. } | Operator::TypedSelectMulti { .. } => {
                Category::Select
            }
            // `Operator` is `#[non_exhaustive]`, so we need a catch-all; reaching
            // it means we encountered an instruction we don't categorize yet.
            _ => bail!("unknown instruction: {op:?}"),
        })
    }
}
