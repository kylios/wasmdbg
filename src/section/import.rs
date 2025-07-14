use std::io::{BufReader, Read};
use std::fmt::Display;

use crate::parseable::{Parseable, Result};
use crate::types::leb128::Leb128;
use crate::types::primitives::{Size, TypeIdx};
use crate::types::enums::{Limits, RefType, ValType, Mut};
use crate::section::Section;

// TODO: move all these types into types.rs
// TODO: implement Display trait for these types

struct GlobalType {
    t: ValType,
    r#mut: Mut
}

impl Parseable for GlobalType {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
        where
            Self: Sized {
        
        let t = ValType::parse(reader)?;
        let r#mut = Mut::parse(reader)?;

        Ok(GlobalType { 
            t,
            r#mut
        })
    }
}

struct MemType {
    lim: Limits
}

impl Parseable for MemType {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
        where
            Self: Sized {
        
        let lim = Limits::parse(reader)?;

        Ok(MemType {
            lim
        })
    }
}

struct TableType {
    et: RefType,
    lim: Limits 
}

impl Parseable for TableType {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
        where
            Self: Sized {
        
        let et = RefType::parse(reader)?;
        let lim = Limits::parse(reader)?;

        Ok(TableType {
            et,
            lim
        })
    }
}

pub struct ImportDesc {
    func: TypeIdx,
    table: TableType,
    mem: MemType,
    global: GlobalType
}

impl Parseable for ImportDesc {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
        where
            Self: Sized {
                
        let func = TypeIdx::parse(reader)?;
        let table = TableType::parse(reader)?;
        let mem = MemType::parse(reader)?;
        let global = GlobalType::parse(reader)?;
        
        Ok(ImportDesc {
            func,
            table,
            mem,
            global
        })
    }
}

pub struct Import {
    // module name
    name: String,
    // name
    nm: String,
    d: ImportDesc
}

impl Parseable for Import {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
        where
            Self: Sized {
        
        let name = String::parse(reader)?;
        let nm = String::parse(reader)?;
        let d = ImportDesc::parse(reader)?;
        
        Ok(Import {
            name: name,
            nm: nm,
            d: d
        })
    }
}

pub struct ImportSec {
    size: Size,
    ims: Vec<Import>
}

impl Section for ImportSec {
    fn section_type(&self) -> &str {
        "import"
    }
    
    fn size(&self) -> Size {
        self.size
    }
}

impl Parseable for ImportSec {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
        where
            Self: Sized {
                
        let size = Leb128::<u32>::parse(reader)?.val;
        
        Ok(ImportSec {
            size: size,
            ims: Vec::<Import>::parse(reader)?
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
