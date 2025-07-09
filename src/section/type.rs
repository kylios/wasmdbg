use std::io::{BufReader, Read};

use crate::parseable::{Parseable, Result};
use crate::types::{Size, FuncType};
use crate::section::Section;

pub struct TypeSection {
    size: Size,
    funcs: Vec<FuncType>
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