use std::fmt::Display;

use crate::types::primitives::{ElemIdx, TableIdx};

pub enum TableInstr {
    Get(TableIdx),
    Set(TableIdx),
    Size(TableIdx),
    Grow(TableIdx),
    Fill(TableIdx),
    Copy(TableIdx, TableIdx),
    Init(TableIdx, ElemIdx),
    Drop(ElemIdx)
}

impl Display for TableInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TableInstr::Get(tableidx) => format!("table.get {}", tableidx),
            TableInstr::Set(tableidx) => format!("table.set {}", tableidx),
            TableInstr::Size(tableidx) => format!("table.size {}", tableidx),
            TableInstr::Grow(tableidx) => format!("table.grow {}", tableidx),
            TableInstr::Fill(tableidx) => format!("table.fill {}", tableidx),
            TableInstr::Copy(dst, src) => format!("table.copy {} {}", dst, src),
            TableInstr::Init(tableidx, elemidx) => format!("table.init {} {}", tableidx, elemidx),
            TableInstr::Drop(elemidx) => format!("elem.drop {}", elemidx)
        };

        write!(f, "{}", s)
    }
}
