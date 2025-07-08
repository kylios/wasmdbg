use std::io::{BufReader, Read};
use std::fmt::Debug;
use std::string::FromUtf8Error;

pub struct ParseError {
    msg: String
}

impl ParseError {
    pub fn new(msg: &str) -> ParseError {
        ParseError {
            msg: msg.to_string()
        }
    }
    
    pub fn to_string(&self) -> &str {
        self.msg.as_str()
    }
}

impl From<std::io::Error> for ParseError {
    fn from(value: std::io::Error) -> Self {
        ParseError {
            msg: value.to_string()
        }
    }
}

impl From<FromUtf8Error> for ParseError {
    fn from(value: FromUtf8Error) -> Self {
        ParseError { msg: value.to_string() } 
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg.as_str())
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;

pub trait Parseable {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized;
}