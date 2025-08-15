use std::fmt::Display;

use crate::types::mem_arg::MemArg;
use crate::types::primitives::{DataIdx, LaneIdx};

pub enum MemoryInstr {
    I32Load(MemArg),
    F32Load(MemArg),
    V128Load(MemArg),
    I32Store(MemArg),
    F32Store(MemArg),
    V128Store(MemArg),
    I32Load8U(MemArg),
    I32Load8S(MemArg),
    I32Load16U(MemArg),
    I32Load16S(MemArg),
    I64Load32U(MemArg),
    I64Load32S(MemArg),
    I32Store8(MemArg),
    I32Store16(MemArg),
    I64Store32(MemArg),
    V128Load8x8U(MemArg),
    V128Load8x8S(MemArg),
    V128Load16x4U(MemArg),
    V128Load16x4S(MemArg),
    V128Load32x2U(MemArg),
    V128Load32x2S(MemArg),
    V128Load32Zero(MemArg),
    V128Load64Zero(MemArg),
    V128Load8Splat(MemArg),
    V128Load16Splat(MemArg),
    V128Load32Splat(MemArg),
    V128Load64Splat(MemArg),
    V128Load8Lane(MemArg, LaneIdx),
    V128Load16Lane(MemArg, LaneIdx),
    V128Load32Lane(MemArg, LaneIdx),
    V128Load64Lane(MemArg, LaneIdx),
    V128Store8Lane(MemArg, LaneIdx),
    V128Store16Lane(MemArg, LaneIdx),
    V128Store32Lane(MemArg, LaneIdx),
    V128Store64Lane(MemArg, LaneIdx),
    MemorySize,
    MemoryGrow,
    MemoryFill,
    MemoryCopy,
    Init(DataIdx),
    DataDrop(DataIdx)
}

impl Display for MemoryInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Memory instr: not implemented>")
    }
}
