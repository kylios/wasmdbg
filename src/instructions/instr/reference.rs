use std::fmt::Display;

pub enum ReferenceInstr {}

impl Display for ReferenceInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Reference instr: not implemented>")
    }
}
