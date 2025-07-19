use std::io::{BufReader, Read};

use crate::parseable::{Parseable, Result};
use crate::types::leb128::Leb128;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_limits() {
        let bytes: [u8; 5] = [0x01, 0x02, 0xE5, 0x8E, 0x26];
        let mut reader = BufReader::new(Cursor::new(bytes));
        let result = Limits::parse(&mut reader);
        assert!(result.is_ok());
        let val = result.expect("The parsed value");
        assert_eq!(
            val,
            Limits {
                min: 2,
                max: Some(624485)
            }
        );

        let bytes: [u8; 4] = [0x00, 0xE5, 0x8E, 0x26];
        let mut reader = BufReader::new(Cursor::new(bytes));
        let result = Limits::parse(&mut reader);
        assert!(result.is_ok());
        let val = result.expect("The parsed value");
        assert_eq!(
            val,
            Limits {
                min: 624485,
                max: None
            }
        )
    }
}
