use std::fmt::Display;
use std::io::{BufReader, Read};

use crate::parseable::{Asked, Parseable, Received, Result};
use crate::section::Section;
use crate::types::leb128::Leb128;
use crate::types::mem_type::MemType;
use crate::types::primitives::Size;

pub struct MemSec {
    size: Size,
    mems: Vec<MemType>,
}

impl Section for MemSec {
    fn section_type(&self) -> &str {
        "memory"
    }

    fn size(&self) -> Size {
        self.size
    }
}

impl MemSec {
    pub fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        let size = u32::from(Leb128::<u32>::parse(reader)?);

        let mems = Vec::<MemType>::parse(reader)?;
        Ok(MemSec {
            size: Size::from(size),
            mems,
        })
    }
}

impl Display for MemSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Size: {}", self.size)?;
        writeln!(f, "Mems:")?;
        for mem in &self.mems {
            writeln!(f, "* {}", mem)?;
        }
        Ok(())
    }
}
