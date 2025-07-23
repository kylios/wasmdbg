use std::fmt::Display;
use std::io::{BufReader, Read};

use crate::parseable::{Asked, ParseError, Parseable, Received, Result};
use crate::types::num_type::{NumType, IType, FType};
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
                0x7f => Ok(ValType::Num(NumType::I(IType::I32))),
                0x7e => Ok(ValType::Num(NumType::I(IType::I64))),
                0x7d => Ok(ValType::Num(NumType::F(FType::F32))),
                0x7c => Ok(ValType::Num(NumType::F(FType::F64))),
                0x7b => Ok(ValType::Vec(VecType::V128)),
                0x70 => Ok(ValType::Ref(RefType::Func)),
                0x6f => Ok(ValType::Ref(RefType::Extern)),
                _ => Err(ParseError::Other("Value is not ValType".to_string())),
            },
            n => Err(ParseError::wrong_num_bytes_read(Asked(1), Received(n))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Cursor};

    #[test]
    fn test_valtype() {
        let bytes: [u8; 8] = [0x6f, 0x70, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x60];
        let mut reader = BufReader::new(Cursor::new(bytes));

        let result = ValType::parse(&mut reader);
        assert!(result.is_ok());
        let result = result.expect("Expected a parsed FuncType");
        assert_eq!(result, ValType::Ref(RefType::Extern));

        let result = ValType::parse(&mut reader);
        assert!(result.is_ok());
        let result = result.expect("Expected a parsed FuncType");
        assert_eq!(result, ValType::Ref(RefType::Func));

        let result = ValType::parse(&mut reader);
        assert!(result.is_ok());
        let result = result.expect("Expected a parsed FuncType");
        assert_eq!(result, ValType::Vec(VecType::V128));

        let result = ValType::parse(&mut reader);
        assert!(result.is_ok());
        let result = result.expect("Expected a parsed FuncType");
        assert_eq!(result, ValType::Num(NumType::F(FType::F64)));

        let result = ValType::parse(&mut reader);
        assert!(result.is_ok());
        let result = result.expect("Expected a parsed FuncType");
        assert_eq!(result, ValType::Num(NumType::F(FType::F32)));

        let result = ValType::parse(&mut reader);
        assert!(result.is_ok());
        let result = result.expect("Expected a parsed FuncType");
        assert_eq!(result, ValType::Num(NumType::I(IType::I64)));

        let result = ValType::parse(&mut reader);
        assert!(result.is_ok());
        let result = result.expect("Expected a parsed FuncType");
        assert_eq!(result, ValType::Num(NumType::I(IType::I32)));

        let result = ValType::parse(&mut reader);
        assert!(result.is_err());
    }
}
