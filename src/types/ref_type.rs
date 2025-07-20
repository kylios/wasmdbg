use std::fmt::Display;
use std::io::{BufReader, Read};

use crate::parseable::{Asked, ParseError, Parseable, Received, Result};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RefType {
    Func,
    Extern,
}

impl Parseable for RefType {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut buf: [u8; 1] = [0; 1];
        let n = reader.read(&mut buf)?;
        match n {
            1 => match u8::from_le_bytes(buf) {
                0x70 => Ok(RefType::Func),
                0x6f => Ok(RefType::Extern),
                _ => Err(ParseError::new("Value is not RefType".to_string())),
            },
            n => Err(ParseError::wrong_num_bytes_read(Asked(1), Received(n))),
        }
    }
}

impl Display for RefType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RefType::Func => write!(f, "func"),
            RefType::Extern => write!(f, "extern"),
        }
    }
}
