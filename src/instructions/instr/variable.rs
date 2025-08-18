use std::fmt::Display;

use crate::types::primitives::{GlobalIdx, LocalIdx};

pub enum VariableInstr {
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    GlobalSet(GlobalIdx)
}

impl Display for VariableInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            VariableInstr::LocalGet(localidx) => format!("local.get {}", localidx),
            VariableInstr::LocalSet(localidx) => format!("local.set {}",localidx),
            VariableInstr::LocalTee(localidx) => format!("local.tee {}", localidx),
            VariableInstr::GlobalGet(globalidx) => format!("global.get {}", globalidx),
            VariableInstr::GlobalSet(globalidx) => format!("global.set {}", globalidx)
        };

        write!(f, "{}", s)
    }
}
