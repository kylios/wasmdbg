use std::fmt::Display;

use crate::types::mem_arg::MemArg;
use crate::types::primitives::{DataIdx, LaneIdx};

pub enum MemoryInstr {
    I32Load(MemArg), I64Load(MemArg), F32Load(MemArg), F64Load(MemArg),
    I32Load8U(MemArg), I32Load8S(MemArg), I32Load16U(MemArg), I32Load16S(MemArg),
    I64Load8U(MemArg), I64Load8S(MemArg), I64Load16U(MemArg), I64Load16S(MemArg), I64Load32U(MemArg), I64Load32S(MemArg),
    I32Store(MemArg), I64Store(MemArg),
    F32Store(MemArg), F64Store(MemArg),
    I32Store8(MemArg), I32Store16(MemArg),
    I64Store8(MemArg), I64Store16(MemArg), I64Store32(MemArg),

    V128Load(MemArg),
    V128Load8x8U(MemArg), V128Load8x8S(MemArg),
    V128Load16x4U(MemArg), V128Load16x4S(MemArg),
    V128Load32x2U(MemArg), V128Load32x2S(MemArg),
    V128Load32Zero(MemArg), V128Load64Zero(MemArg),
    V128Load8Splat(MemArg), V128Load16Splat(MemArg), V128Load32Splat(MemArg), V128Load64Splat(MemArg),
    V128Load8Lane(MemArg, LaneIdx), V128Load16Lane(MemArg, LaneIdx), V128Load32Lane(MemArg, LaneIdx), V128Load64Lane(MemArg, LaneIdx),
    V128Store(MemArg),
    V128Store8Lane(MemArg, LaneIdx),
    V128Store16Lane(MemArg, LaneIdx),
    V128Store32Lane(MemArg, LaneIdx),
    V128Store64Lane(MemArg, LaneIdx),
    MemorySize, MemoryGrow, MemoryFill, MemoryCopy,
    Init(DataIdx),
    DataDrop(DataIdx)
}

impl Display for MemoryInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MemoryInstr::I32Load(mem_arg) => format!("i32.load {}", mem_arg),
            MemoryInstr::I64Load(mem_arg) => format!("i64.load {}", mem_arg),
            MemoryInstr::F32Load(mem_arg) => format!("f32.load {}", mem_arg),
            MemoryInstr::F64Load(mem_arg) => format!("f64.load {}", mem_arg),
            MemoryInstr::I32Load8U(mem_arg) => format!("i32.load8_u {}", mem_arg),
            MemoryInstr::I32Load8S(mem_arg) => format!("i32.load8_s {}", mem_arg),
            MemoryInstr::I32Load16U(mem_arg) => format!("i32.load16_u {}", mem_arg),
            MemoryInstr::I32Load16S(mem_arg) => format!("i32.load16_s {}", mem_arg),
            MemoryInstr::I64Load8U(mem_arg) => format!("i64.load8_u {}", mem_arg),
            MemoryInstr::I64Load8S(mem_arg) => format!("i64.load8_s {}", mem_arg),
            MemoryInstr::I64Load16U(mem_arg) => format!("i64.load16_u {}", mem_arg),
            MemoryInstr::I64Load16S(mem_arg) => format!("i64.load16_s {}", mem_arg),
            MemoryInstr::I64Load32U(mem_arg) => format!("i64.load32_u {}", mem_arg),
            MemoryInstr::I64Load32S(mem_arg) => format!("i64.load32_s {}", mem_arg),
            MemoryInstr::I32Store(mem_arg) => format!("i32.store {}", mem_arg),
            MemoryInstr::I64Store(mem_arg) => format!("i64.store {}", mem_arg),
            MemoryInstr::F32Store(mem_arg) => format!("f32.store {}", mem_arg),
            MemoryInstr::F64Store(mem_arg) => format!("f64.store {}", mem_arg),
            MemoryInstr::I32Store8(mem_arg) => format!("i32.store8 {}", mem_arg),
            MemoryInstr::I32Store16(mem_arg) => format!("i32.store16 {}", mem_arg),
            MemoryInstr::I64Store8(mem_arg) => format!("i64.store8 {}", mem_arg),
            MemoryInstr::I64Store16(mem_arg) => format!("i64.store16 {}", mem_arg),
            MemoryInstr::I64Store32(mem_arg) => format!("i64.store32 {}", mem_arg),
            MemoryInstr::MemorySize => format!("memory.size"),
            MemoryInstr::MemoryGrow => format!("memory.grow"),
            MemoryInstr::Init(dataidx) => format!("memory.init {}", dataidx),
            MemoryInstr::DataDrop(dataidx) => format!("data.drop {}", dataidx),
            MemoryInstr::MemoryCopy => format!("memory.copy"),
            MemoryInstr::MemoryFill => format!("memory.fill"),
            MemoryInstr::V128Load(mem_arg) => format!("v128.load {}", mem_arg),
            MemoryInstr::V128Load8x8S(mem_arg) => format!("v128.load8x8_s {}", mem_arg),
            MemoryInstr::V128Load8x8U(mem_arg) => format!("v128.load8x8_u {}", mem_arg),
            MemoryInstr::V128Load16x4S(mem_arg) => format!("v128.load16x4_s {}", mem_arg),
            MemoryInstr::V128Load16x4U(mem_arg) => format!("v128.load16x4_u {}", mem_arg),
            MemoryInstr::V128Load32x2S(mem_arg) => format!("v128.load32x2_s {}", mem_arg),
            MemoryInstr::V128Load32x2U(mem_arg) => format!("v128.load32x2_u {}", mem_arg),
            MemoryInstr::V128Load8Splat(mem_arg) => format!("v128.load8_splat {}", mem_arg),
            MemoryInstr::V128Load16Splat(mem_arg) => format!("v128.load16_splat {}", mem_arg),
            MemoryInstr::V128Load32Splat(mem_arg) => format!("v128.load32_splat {}", mem_arg),
            MemoryInstr::V128Load64Splat(mem_arg) => format!("v128.load64_splat {}", mem_arg),
            MemoryInstr::V128Store(mem_arg) => format!("v128.store {}", mem_arg),
            MemoryInstr::V128Load8Lane(mem_arg, laneidx) => format!("v128.load8_lane {} {}", mem_arg, laneidx),
            MemoryInstr::V128Load16Lane(mem_arg, laneidx) => format!("v128.load16_lane {} {}", mem_arg, laneidx),
            MemoryInstr::V128Load32Lane(mem_arg, laneidx) => format!("v128.load32_lane {} {}", mem_arg, laneidx),
            MemoryInstr::V128Load64Lane(mem_arg, laneidx) => format!("v128.load64_lane {} {}", mem_arg, laneidx),
            MemoryInstr::V128Store8Lane(mem_arg, laneidx) => format!("v128.store8_lane {} {}", mem_arg, laneidx),
            MemoryInstr::V128Store16Lane(mem_arg, laneidx) => format!("v128.store16_lane {} {}", mem_arg, laneidx),
            MemoryInstr::V128Store32Lane(mem_arg, laneidx) => format!("v128.store32_lane {} {}", mem_arg, laneidx),
            MemoryInstr::V128Store64Lane(mem_arg, laneidx) => format!("v128.store64_lane {} {}", mem_arg, laneidx),
            MemoryInstr::V128Load32Zero(mem_arg) => format!("v128.load32_zero {}", mem_arg),
            MemoryInstr::V128Load64Zero(mem_arg) => format!("v128.load64_zero {}", mem_arg)
        };

        write!(f, "{}", s)
    }
}
