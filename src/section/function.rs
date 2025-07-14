use std::io::{BufReader, Read};
use std::fmt::Display;

use crate::parseable::{Parseable, Result};
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

impl Parseable for FunctionSec {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
        where
            Self: Sized {
                        
        let size = Leb128::<Size>::parse(reader)?.val;

        // TODO: this is temporary code and should be replaced by
        // actual parsing. We are just consuming bytes for the sake
        // of testing!
        let bytes_remaining = usize::try_from(size).unwrap();
        let mut bytes_read = 0;
        let mut buf: [u8; 1] = [0; 1];
        loop {
            let n = reader.read(&mut buf)?; 
            if n != 1 {
                panic!("Should have read 1 byte");
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
