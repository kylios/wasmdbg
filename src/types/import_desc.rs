use std::io::BufReader;

use crate::parseable::{Parseable, Result, ReadSeek};
use crate::types::global_type::GlobalType;
use crate::types::mem_type::MemType;
use crate::types::primitives::TypeIdx;
use crate::types::table_type::TableType;

pub struct ImportDesc {
    func: TypeIdx,
    table: TableType,
    mem: MemType,
    global: GlobalType,
}

impl Parseable for ImportDesc {
    fn parse(reader: &mut BufReader<dyn ReadSeek>) -> Result<Self>
    where
        Self: Sized,
    {
        let func = TypeIdx::parse(reader)?;
        let table = TableType::parse(reader)?;
        let mem = MemType::parse(reader)?;
        let global = GlobalType::parse(reader)?;

        Ok(ImportDesc {
            func,
            table,
            mem,
            global,
        })
    }
}
