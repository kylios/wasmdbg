use std::fmt::Display;

pub enum TableInstr {}

impl Display for TableInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Table instr: not implemented>")
    }
}
