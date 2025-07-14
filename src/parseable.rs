use std::io::{BufReader, Read};
use std::fmt::Debug;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub struct Asked(pub usize);
#[derive(Debug, Clone, Copy)]
pub struct Received(pub usize);

impl From<Received> for usize {
    fn from(value: Received) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub enum ParseError {
    WrongNumBytesRead(Asked, Received),
    Other(String)    
}

impl ParseError {
    pub fn wrong_num_bytes_read(asked: Asked, received: Received) -> ParseError {
        ParseError::WrongNumBytesRead(asked, received)
    }
}

impl ParseError {
    pub fn new(msg: String) -> ParseError {
        ParseError::Other(msg)
    }
}

impl From<std::io::Error> for ParseError {
    fn from(value: std::io::Error) -> Self {
        ParseError::Other(value.to_string())
    }
}

impl From<FromUtf8Error> for ParseError {
    fn from(value: FromUtf8Error) -> Self {
        ParseError::Other(value.to_string())
    }
}


pub type Result<T> = std::result::Result<T, ParseError>;

pub trait Parseable {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized;
}