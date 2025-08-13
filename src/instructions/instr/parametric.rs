use std::fmt::Display;

pub enum ParametricInstr {}

impl Display for ParametricInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Parametric instr: not implemented>")
    }
}
