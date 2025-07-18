use std::io::{BufReader, Read};

use crate::parseable::{Parseable, Result};
use crate::types::leb128::Leb128;

pub struct Limits {
    min: u32,
    max: Option<u32>,
}

impl Parseable for Limits {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        let flag = u8::parse(reader)?;
        let min = u32::from(Leb128::<u32>::parse(reader)?);
        let mut max: Option<u32> = None;
        if flag == 0x1 {
            let max_val = u32::from(Leb128::<u32>::parse(reader)?);
            max = Some(max_val);
        }

        Ok(Limits { min, max })
    }
}
