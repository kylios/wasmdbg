use std::fmt::Display;

pub enum MemoryInstr {}

impl Display for MemoryInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Memory instr: not implemented>")
    }
}
