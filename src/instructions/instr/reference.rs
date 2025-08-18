use std::fmt::Display;

use crate::types::{primitives::FuncIdx, ref_type::RefType};

pub enum ReferenceInstr {
    Null(RefType),
    IsNull,
    Func(FuncIdx)
}

impl Display for ReferenceInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ReferenceInstr::Null(reftype) => format!("ref.null {}", reftype),
            ReferenceInstr::IsNull => "ref.is_null".to_string(),
            ReferenceInstr::Func(funcidx) => format!("ref.func {}", funcidx)
        };

        write!(f, "{}", s)
    }
}
