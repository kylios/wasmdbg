use std::fmt::Display;

use crate::types::val_type::ValType;

pub enum ParametricInstr {
    Drop,
    Select(Option<ValType>)
}

impl Display for ParametricInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ParametricInstr::Drop => "drop".to_string(),
            ParametricInstr::Select(valtype) => match valtype {
                Some(valtype) => format!("select {}", valtype),
                None => "select".to_string()
            }
        };

        write!(f, "{}", s)
    }
}
