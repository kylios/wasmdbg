use std::io::{BufReader, Read};
use std::fmt::Display;

use crate::parseable::{Parseable, Result};
use crate::types::leb128::Leb128;
use crate::types::primitives::Size;
use crate::types::enums::FuncType;
use crate::section::Section;

pub struct TypeSec {
    size: Size,
    funcs: Vec<FuncType>
}

impl Display for TypeSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Size: {}", self.size)?;
        writeln!(f, "Funcs:")?;
        for functype in &self.funcs {
            writeln!(f, "{}", functype)?;
        }

        Ok(())
    }
}

impl Section for TypeSec {
    fn section_type(&self) -> &str {
        "type"
    }
    
    fn size(&self) -> Size {
        self.size
    }
}

impl Parseable for TypeSec {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
        where
            Self: Sized {
        
        let size = u32::from(Leb128::<Size>::parse(reader)?);

        Ok(TypeSec {
            size: size,
            funcs: Vec::<FuncType>::parse(reader)?
        })
    }
}
