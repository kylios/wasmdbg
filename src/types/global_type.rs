use std::io::{BufReader, Read};

use crate::parseable::{Parseable, Result};
use crate::types::r#mut::Mut;
use crate::types::val_type::ValType;

pub struct GlobalType {
    t: ValType,
    r#mut: Mut,
}

impl Parseable for GlobalType {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        let t = ValType::parse(reader)?;
        let r#mut = Mut::parse(reader)?;

        Ok(GlobalType { t, r#mut })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
}
