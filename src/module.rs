use std::io::{self, BufReader, Read};

use crate::parseable::{Parseable, Result, ParseError};

pub struct Module {
    pub version: u32
}

impl Module {
    fn parse_magic(reader: &mut BufReader<dyn Read>) -> Result<()> {
        let mut buffer = [0; 4];
        let n = reader.read(&mut buffer[..])?;
        if n != 4 {
            let mut owned_string = "Tried to read 4 bytes of magic number. Read ".to_owned();
            owned_string.push_str(n.to_string().as_str());
            return Err(ParseError::new(&owned_string))
        }
        
        // `0x00 asm` in ASCII
        if buffer != [0, 97, 115, 109] {
            return Err(ParseError::new("Bad magic"))
        }
        
        Ok(())
    }
    
    fn parse_version(reader: &mut BufReader<dyn Read>) -> Result<u32> {
        let version = u32::parse(reader)?;
        Ok(version)
    }
}

impl Parseable for Module {
    fn parse(reader: &mut std::io::BufReader<dyn std::io::Read>) -> Result<Module> {
        Self::parse_magic(reader)?;
        let version = Self::parse_version(reader)?;
        
        if version != 1 {
            return Err(ParseError::new("Wasm version should be 1"))
        }
        Ok(Module {
            version: version
        })
    }
}