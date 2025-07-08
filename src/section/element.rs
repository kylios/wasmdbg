use std::io::{BufReader, Read};

use crate::parseable::Result;
use crate::types::Size;
use crate::section::Section;

pub struct ElementSection {
    size: Size
}

impl Section for ElementSection {
    fn section_type(&self) -> &str {
        "element"
    }
    
    fn size(&self) -> Size {
        self.size
    }
}

pub fn parse(size: Size, reader: &mut BufReader<dyn Read>) -> Result<Box<dyn Section>> {
    let section = ElementSection {
        size: size
    };

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
    Ok(Box::from(section))
}