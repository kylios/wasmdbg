use std::fmt::Display;
use std::io::{BufReader, Read};
use std::result::Result;

use crate::parseable::{Asked, ParseError, Parseable, Received};
use crate::section::Section;
use crate::types::leb128::Leb128;
use crate::types::primitives::Size;

pub struct DataCountSec {
    size: Size,
}

impl Section for DataCountSec {
    fn section_type(&self) -> &str {
        "data_count"
    }

    fn size(&self) -> Size {
        self.size
    }
}

impl DataCountSec {
    pub fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let size = u32::from(Leb128::<u32>::parse(reader)?);

        // TODO: this is temporary code and should be replaced by
        // actual parsing. We are just consuming bytes for the sake
        // of testing!
        let bytes_remaining = usize::try_from(size).unwrap();
        let mut bytes_read = 0;
        let mut buf: [u8; 1] = [0; 1];
        loop {
            let n = reader.read(&mut buf)?;
            if n != 1 {
                return Err(ParseError::WrongNumBytesRead(Asked(1), Received(n)));
            }

            bytes_read += n;
            if bytes_read == bytes_remaining {
                break;
            }
        }

        Ok(DataCountSec { size: Size(size) })
    }
}

impl Display for DataCountSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Size: {}", self.size)
    }
}
