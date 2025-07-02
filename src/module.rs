use std::io::{self, BufReader, Read};

use crate::parseable::Parseable;

pub struct Module {
    pub version: u32
}

impl Module {
    fn parse_magic(reader: &mut BufReader<dyn Read>) -> io::Result<()> {
        let mut buffer = [0; 4];
        let n = reader.read(&mut buffer[..])?;
        assert_eq!(n, 4);
        
        // x00 'asm'
        // TODO: use return value to return error instead of asserting
        assert_eq!(buffer, [0, 97, 115, 109]);
        
        Ok(())
    }
    
    fn parse_version(reader: &mut BufReader<dyn Read>) -> u32 {
        let version = u32::parse(reader);
        version
    }
}

impl Parseable for Module {
    fn parse(reader: &mut std::io::BufReader<dyn std::io::Read>) -> Self {
        Self::parse_magic(reader);
        let version = Self::parse_version(reader);
        // TODO: probably better to use return value for error
        // handling instead of asserting
        assert_eq!(version, 1);
        Module {
            version: version
        }
    }
}