use std::fmt::Display;

pub enum VariableInstr {}

impl Display for VariableInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Valiable instr: not implemented>")
    }
}
