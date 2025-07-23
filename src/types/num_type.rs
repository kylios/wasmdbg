use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IType {
    I32,
    I64
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FType {
    F32,
    F64
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NumType {
    I(IType),
    F(FType)
}

impl Display for NumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumType::I(IType::I32) => write!(f, "i32"),
            NumType::I(IType::I64) => write!(f, "i64"),
            NumType::F(FType::F32) => write!(f, "f32"),
            NumType::F(FType::F64) => write!(f, "f64"),
        }
    }
}
