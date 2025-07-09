use std::io::{BufReader, Read};
use std::fmt::Display;

use crate::parseable::{Parseable, Result};
use crate::types::{Size, FuncType};
use crate::section::Section;

pub struct TypeSection {
    size: Size,
    funcs: Vec<FuncType>
}

impl Display for TypeSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Size: {}", self.size)?;
        writeln!(f, "Funcs:")?;
        for functype in &self.funcs {
            writeln!(f, "{}", functype)?;
        }

        Ok(())
    }
}

impl Section for TypeSection {
    fn section_type(&self) -> &str {
        "type"
    }
    
    fn size(&self) -> Size {
        self.size
    }
}

pub fn parse(size: Size, reader: &mut BufReader<dyn Read>) -> Result<Box<dyn Section>> {
    
    let funcs = Vec::<FuncType>::parse(reader)?;

    let section = TypeSection {
        size: size,
        funcs: funcs
    };

    Ok(Box::from(section))
}