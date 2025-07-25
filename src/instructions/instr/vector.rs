use std::fmt::Display;

enum IShape {
    I8x16,
    I16x8,
    I32x4,
    I64x2
}

enum FShape {
    F32x4,
    F64x2
}

enum Shape {
    I(IShape),
    F(FShape)
}

enum Half {
    Low,
    High
}

pub enum VectorInstr {
    // TODO: not implemented
}

impl Display for VectorInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Vector instr: not implemented>")
    }
}
