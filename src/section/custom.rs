use std::io::{BufReader, Read as IoRead};
use std::fmt::Display;
use std::result::Result;

use crate::parseable::{Asked, ParseError, Parseable, Received};
use crate::types::primitives::Size;
use crate::types::leb128::Leb128;
use crate::section::{Section, SectionParseError};

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

pub struct Read(usize);
pub struct Remaining(usize);

pub enum CustomSecParseError {
    Parse(ParseError),
    IoError(std::io::Error),
    ByteCount(Read, Remaining)
}

impl From<ParseError> for CustomSecParseError {
    fn from(value: ParseError) -> Self {
        CustomSecParseError::Parse(value)
    }
}

impl From<std::io::Error> for CustomSecParseError {
    fn from(value: std::io::Error) -> Self {
        CustomSecParseError::IoError(value)
    }
}

impl Into<SectionParseError> for CustomSecParseError {
    fn into(self) -> SectionParseError {
        let s: String = match self {
            CustomSecParseError::Parse(e) => e.to_string(),
            CustomSecParseError::IoError(e) => e.to_string(),
            CustomSecParseError::ByteCount(read, remaining) => {
                format!("Should have read {}, but read {}", read.0, remaining.0)
            }
        };
        SectionParseError(s)
    }
}
impl CustomSec {
    pub fn parse(reader: &mut BufReader<dyn IoRead>) -> Result<Self, CustomSecParseError>
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
                return Err(CustomSecParseError::Parse(ParseError::WrongNumBytesRead(Asked(1), Received(n))));
            }
            data.extend_from_slice(&buf);
            
            bytes_read += n;
            if bytes_read == bytes_remaining {
                break;
            } 
        }

        if bytes_read != bytes_remaining {
            return Err(CustomSecParseError::ByteCount(Read(bytes_read), Remaining(bytes_remaining)))
        }

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
