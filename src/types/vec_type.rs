use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VecType {
    V128,
}

impl Display for VecType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VecType::V128 => write!(f, "v128"),
        }
    }
}
