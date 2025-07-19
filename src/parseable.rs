use std::fmt::{Debug, Display};
use std::io::{BufReader, Read};
use std::string::FromUtf8Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Asked(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Received(pub usize);

impl From<Received> for usize {
    fn from(value: Received) -> Self {
        value.0
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    WrongNumBytesRead(Asked, Received),
    Other(String),
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

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::WrongNumBytesRead(asked, received) => {
                write!(
                    f,
                    "Requested {} bytes; received {} bytes",
                    asked.0, received.0
                )
            }
            ParseError::Other(e) => write!(f, "{}", e),
        }
    }
}
