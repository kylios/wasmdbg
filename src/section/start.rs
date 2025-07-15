use std::io::{BufReader, Read};
use std::fmt::Display;

use crate::parseable::{Asked, Parseable, Received, Result};
use crate::types::leb128::Leb128;
use crate::types::primitives::Size;
use crate::section::Section;

pub struct StartSec {
    size: Size
}

impl Section for StartSec {
    fn section_type(&self) -> &str {
        "start"
    }
    
    fn size(&self) -> Size {
        self.size
    }
}

impl Display for StartSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Size: {}", self.size)
    }
}

impl StartSec {
    pub fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
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
                return Err(crate::parseable::ParseError::WrongNumBytesRead(Asked(1), Received(n)))
            }
            
            bytes_read += n;
            if bytes_read == bytes_remaining {
                break;
            } 
        }       
        
        Ok(StartSec { size: size })
    }
}
