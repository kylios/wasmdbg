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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    use crate::parseable::ParseError;

    #[test]
    fn test_reftype() {
        let bytes: [u8; 3] = [0x70, 0x6f, 0x6e];
        let mut reader = BufReader::new(Cursor::new(bytes));
        let result = RefType::parse(&mut reader);
        assert!(result.is_ok());
        let result = result.expect("The parsed value");
        assert_eq!(result, RefType::Func);

        let result = RefType::parse(&mut reader);
        assert!(result.is_ok());
        let result = result.expect("The parsed value");
        assert_eq!(result, RefType::Extern);

        let result = RefType::parse(&mut reader);
        assert!(result.is_err());
        let result = result.expect_err("A parse error");
        assert_eq!(result, ParseError::new("Value is not RefType".to_string()));
    }
}
