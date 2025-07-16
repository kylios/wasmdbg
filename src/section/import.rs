use std::fmt::Display;
use std::io::{BufReader, Read};
use std::result::Result;

use crate::parseable::{ParseError, Parseable};
use crate::section::Section;
use crate::types::enums::ImportDesc;
use crate::types::leb128::Leb128;
use crate::types::primitives::Size;

pub struct Import {
    // module name
    name: String,
    // name
    nm: String,
    d: ImportDesc,
}

impl Parseable for Import {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let name = String::parse(reader)?;
        let nm = String::parse(reader)?;
        let d = ImportDesc::parse(reader)?;

        Ok(Import {
            name: name,
            nm: nm,
            d: d,
        })
    }
}

pub struct ImportSec {
    size: Size,
    ims: Vec<Import>,
}

impl Section for ImportSec {
    fn section_type(&self) -> &str {
        "import"
    }

    fn size(&self) -> Size {
        self.size
    }
}

impl ImportSec {
    pub fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let size = u32::from(Leb128::<u32>::parse(reader)?);

        Ok(ImportSec {
            size: Size(size),
            ims: Vec::<Import>::parse(reader)?,
        })
    }
}

impl Display for ImportSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Size: {}", self.size)?;
        writeln!(f, "Imports:")?;
        for im in &self.ims {
            writeln!(f, "* im: {}/{}", im.name, im.nm)?;
        }

        Ok(())
    }
}
