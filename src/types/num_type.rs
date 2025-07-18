use std::fmt::Display;

pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

impl Display for NumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumType::I32 => write!(f, "i32"),
            NumType::I64 => write!(f, "i64"),
            NumType::F32 => write!(f, "f32"),
            NumType::F64 => write!(f, "F64"),
        }
    }
}
