use std::fmt::Display;
use std::io::{BufReader, Read};

use crate::parseable::{Asked, ParseError, Parseable, Received, Result};
use crate::types::ref_type::RefType;
use crate::types::result_type::ResultType;

#[derive(Debug, PartialEq)]
pub struct FuncType {
    rt1: ResultType,
    rt2: ResultType,
}

impl FuncType {
    fn parse_first_byte(reader: &mut BufReader<dyn Read>) -> Result<u8> {
        let mut buf: [u8; 1] = [0; 1];
        let n = reader.read(&mut buf)?;
        match n {
            1 => Ok(u8::from_le_bytes(buf)),
            n => Err(ParseError::wrong_num_bytes_read(Asked(1), Received(n))),
        }
    }
}

impl Display for FuncType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "* rt1: ")?;
        for rt in &self.rt1 {
            write!(f, "{}, ", rt)?;
        }
        writeln!(f)?;
        write!(f, "* rt2: ")?;
        for rt in &self.rt2 {
            write!(f, "{}, ", rt)?;
        }
        writeln!(f)?;

        Ok(())
    }
}

impl Parseable for FuncType {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        // First byte is 0x60
        let first_byte = Self::parse_first_byte(reader)?;
        if first_byte != 0x60 {
            return Err(ParseError::Other(
                "First byte of FuncType should be 0x60".to_string(),
            ));
        }

        let rt1 = ResultType::parse(reader)?;
        let rt2 = ResultType::parse(reader)?;

        let func = FuncType { rt1: rt1, rt2: rt2 };

        Ok(func)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{num_type::NumType, val_type::ValType, vec_type::VecType};

    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_func_type() {
        let bytes: [u8; 2] = [0x01, 0x02];
        let mut reader = BufReader::new(Cursor::new(bytes));
        let result = FuncType::parse(&mut reader);
        assert!(result.is_err());
        let err = result.expect_err("Expected an error");
        assert_eq!(
            err,
            ParseError::Other("First byte of FuncType should be 0x60".to_string())
        );

        let bytes: [u8; 10] = [0x60, 0x2, 0x6f, 0x70, 0x5, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f];
        let mut reader = BufReader::new(Cursor::new(bytes));
        let result = FuncType::parse(&mut reader);
        assert!(result.is_ok());
        let result = result.expect("Expected a parsed FuncType");

        let cmp_vec = [ValType::Ref(RefType::Extern), ValType::Ref(RefType::Func)];
        let matching = result
            .rt1
            .iter()
            .zip(&cmp_vec)
            .filter(|&(a, b)| a == b)
            .count();
        assert_eq!(matching, 2);

        let cmp_vec = [
            ValType::Vec(VecType::V128),
            ValType::Num(NumType::F64),
            ValType::Num(NumType::F32),
            ValType::Num(NumType::I64),
            ValType::Num(NumType::I32),
        ];
        let matching = result
            .rt2
            .iter()
            .zip(&cmp_vec)
            .filter(|&(a, b)| a == b)
            .count();
        assert_eq!(matching, 5);
    }
}
