use std::io::{BufReader, Read};

use crate::parseable::{Parseable, Result};
use crate::types::limits::Limits;
use crate::types::ref_type::RefType;

pub struct TableType {
    et: RefType,
    lim: Limits,
}

impl Parseable for TableType {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        let et = RefType::parse(reader)?;
        let lim = Limits::parse(reader)?;

        Ok(TableType { et, lim })
    }
}
