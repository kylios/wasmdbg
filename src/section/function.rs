use std::io::{BufReader, Read};
use std::fmt::Display;
use std::result::Result;

use crate::parseable::{Parseable, ParseError, Asked, Received};
use crate::types::primitives::Size;
use crate::types::leb128::Leb128;
use crate::section::Section;

pub struct FunctionSec {
    size: Size
}

impl Section for FunctionSec {
    fn section_type(&self) -> &str {
        "function"
    }
    
    fn size(&self) -> Size {
        self.size
    }
}

impl FunctionSec {
    pub fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self, ParseError>
        where
            Self: Sized {
                        
        let size = u32::from(Leb128::<Size>::parse(reader)?);

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
     
        Ok(FunctionSec { size: size })
    }
}

impl Display for FunctionSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Size: {}", self.size)
    }
}
