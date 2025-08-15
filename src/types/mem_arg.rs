use std::io::{BufReader, Read};

use crate::types::leb128::Leb128;
use crate::parseable::{Parseable, Result, ReadSeek};

#[derive(Debug, PartialEq, Eq)]
pub struct MemArg {
    offset: u32,
    align: u32,
}

impl Parseable for MemArg {
    fn parse(reader: &mut BufReader<dyn ReadSeek>) -> Result<Self> {
        let offset = Leb128::<u32>::parse(reader)?;
        let align = Leb128::<u32>::parse(reader)?;
        Ok(MemArg {
            offset: u32::from(offset),
            align: u32::from(align)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_mem_arg_parse() {
        let bytes: [u8; 2] = [
            0x6F,
            0x70,
        ];
        let mut reader = BufReader::new(Cursor::new(bytes));
        let result = MemArg::parse(&mut reader);
        assert!(result.is_ok());
        let mem_arg = result.expect("The parsed value");
        assert_eq!(mem_arg.offset, 111);
        assert_eq!(mem_arg.align, 112);
    }
}
