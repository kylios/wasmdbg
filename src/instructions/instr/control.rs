use std::fmt::Display;

use crate::types::primitives::{LabelIdx, TypeIdx, FuncIdx, TableIdx};
use crate::types::val_type::ValType;

/*
 * blocktype := typeidx | valtype
 */
enum BlockType {
    TypeIdx(TypeIdx),
    ValType(ValType)
}

impl Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockType::TypeIdx(t) => write!(f, "{}", t),
            BlockType::ValType(t) => write!(f, "{}", t)
        }
    }
}

/*
 * instr := ...
 *       | nop
 *       | unreachable
 *       | block blocktype instr* end
 *       | loop blocktype instr* end
 *       | if blocktype instr* else instr* end
 *       | br labelidx
 *       | br_if labelidx
 *       | br_table vec(labelidx) labelidx
 *       | return
 *       | call funcidx
 *       | call_indirect tableidx typeidx
 */
pub enum ControlInstr {
    Nop,
    Unreachable,
    Block(BlockType),
    Loop(BlockType),
    If(BlockType),
    Else,
    End,
    Br(LabelIdx),
    BrIf(LabelIdx),
    BrTable(Vec<LabelIdx>, LabelIdx),
    Return,
    Call(FuncIdx),
    CallIndirect(TableIdx, TypeIdx)
}

fn vec_to_string<T: Display>(v: &Vec<T>) -> String {
    v
        .into_iter()
        .map(|l| format!("{} ", l).to_string())
        .collect::<String>()
}

impl Display for ControlInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ControlInstr::Nop => "nop".to_string(),
            ControlInstr::Unreachable => "unreachable".to_string(),
            ControlInstr::Block(t) => format!("block {}", t),
            ControlInstr::Loop(t) => format!("loop {}", t),
            ControlInstr::If(t) => format!("if {}", t),
            ControlInstr::Else => "else".to_string(),
            ControlInstr::End => "end".to_string(),
            ControlInstr::Br(l) => format!("br {}", l),
            ControlInstr::BrIf(l) => format!("br_if {}", l),
            ControlInstr::BrTable(v, l) => format!("br_table {} {}", vec_to_string(&v), l),
            ControlInstr::Return => "return".to_string(),
            ControlInstr::Call(t) => format!("call {}", t),
            ControlInstr::CallIndirect(tab, typ) => format!("call_indirect {} {}", tab, typ)
        };

        write!(f, "{}", s)
    }
}
