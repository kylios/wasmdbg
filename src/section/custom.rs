use std::io::{BufReader, Read};
use std::fmt::Display;

use crate::parseable::{Parseable, Result};
use crate::types::primitives::Size;
use crate::types::leb128::Leb128;
use crate::section::Section;

pub struct CustomSec {
    size: Size,
    name: String,
    data: Vec<u8>    
}

impl Section for CustomSec {
    fn section_type(&self) -> &str {
        "custom"
    }
    
    fn size(&self) -> Size {
        self.size
    }
}

impl Parseable for CustomSec {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
        where
            Self: Sized {
        
        let size = u32::from(Leb128::<Size>::parse(reader)?);
        
        let name = String::parse(reader)?;
        let size_usize = usize::try_from(size).unwrap();
        let bytes_remaining = size_usize - name.len();

        let mut data = Vec::<u8>::with_capacity(bytes_remaining);
        
        let mut bytes_read = 0;
        let mut buf: [u8; 1] = [0; 1];
        loop {
            let n = reader.read(&mut buf)?; 
            if n != 1 {
                panic!("Should have read 1 byte");
            }
            data.extend_from_slice(&buf);
            
            bytes_read += n;
            if bytes_read == bytes_remaining {
                break;
            } 
        }

        assert_eq!(bytes_read, bytes_remaining);

        Ok(CustomSec {
            size: size,
            name: name,
            data: data
        })     
    }
}

impl Display for CustomSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Size: {}", self.size)?;
        writeln!(f, "Name: {}", self.name)?;
        Ok(())
    }
}
