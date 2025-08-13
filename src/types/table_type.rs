use std::io::BufReader;

use crate::parseable::{Parseable, Result, ReadSeek};
use crate::types::limits::Limits;
use crate::types::ref_type::RefType;

pub struct TableType {
    et: RefType,
    lim: Limits,
}

impl Parseable for TableType {
    fn parse(reader: &mut BufReader<dyn ReadSeek>) -> Result<Self>
    where
        Self: Sized,
    {
        let et = RefType::parse(reader)?;
        let lim = Limits::parse(reader)?;

        Ok(TableType { et, lim })
    }
}
