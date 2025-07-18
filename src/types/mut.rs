use std::io::{BufReader, Read};

use crate::parseable::{Asked, ParseError, Parseable, Received, Result};

pub enum Mut {
    Val(u8),
}

pub const CONST: Mut = Mut::Val(0x0);
pub const VAR: Mut = Mut::Val(0x1);

impl Parseable for Mut {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut bytes: [u8; 1] = [0; 1];
        let n = reader.read(&mut bytes)?;
        match n {
            1 => match bytes[0] {
                0x0 => Ok(CONST),
                0x1 => Ok(VAR),
                _ => Err(ParseError::Other("Mut type must be 0x0 or 0x1".to_string())),
            },
            n => Err(ParseError::wrong_num_bytes_read(Asked(1), Received(n))),
        }
    }
}
