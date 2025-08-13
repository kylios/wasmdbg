use std::fmt::Display;
use std::io::BufReader;

use crate::parseable::{Parseable, Result, ReadSeek};
use crate::types::limits::Limits;

pub struct MemType {
    lim: Limits,
}

impl Parseable for MemType {
    fn parse(reader: &mut BufReader<dyn ReadSeek>) -> Result<Self>
    where
        Self: Sized,
    {
        let lim = Limits::parse(reader)?;

        Ok(MemType { lim })
    }
}

impl Display for MemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "memtype: {}", self.lim)
    }
}
