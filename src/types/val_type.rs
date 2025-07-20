use std::fmt::Display;
use std::io::{BufReader, Read};

use crate::parseable::{Asked, ParseError, Parseable, Received, Result};
use crate::types::num_type::NumType;
use crate::types::ref_type::RefType;
use crate::types::vec_type::VecType;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ValType {
    Num(NumType),
    Vec(VecType),
    Ref(RefType),
}

impl Display for ValType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValType::Num(t) => write!(f, "numtype {}", t),
            ValType::Vec(t) => write!(f, "vectype {}", t),
            ValType::Ref(t) => write!(f, "reftype {}", t),
        }
    }
}

impl Parseable for ValType {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut buf: [u8; 1] = [0; 1];
        let n = reader.read(&mut buf)?;
        match n {
            1 => match u8::from_le_bytes(buf) {
                0x7f => Ok(ValType::Num(NumType::I32)),
                0x7e => Ok(ValType::Num(NumType::I64)),
                0x7d => Ok(ValType::Num(NumType::F32)),
                0x7c => Ok(ValType::Num(NumType::F64)),
                0x7b => Ok(ValType::Vec(VecType::V128)),
                0x70 => Ok(ValType::Ref(RefType::Func)),
                0x6f => Ok(ValType::Ref(RefType::Extern)),
                _ => Err(ParseError::Other("Value is not ValType".to_string())),
            },
            n => Err(ParseError::wrong_num_bytes_read(Asked(1), Received(n))),
        }
    }
}
