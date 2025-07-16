use std::fmt::Display;
use std::io::{BufReader, Read};

use crate::parseable::{Asked, ParseError, Parseable, Received, Result};
use crate::types::leb128::Leb128;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Size(pub u32);

impl From<u32> for Size {
    fn from(value: u32) -> Self {
        Size(value)
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Parseable for Size {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Size(u32::parse(reader)?))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TypeIdx(pub u32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FuncIdx(pub u32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TableIdx(pub u32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MemIdx(pub u32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GlobalIdx(pub u32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ElemIdx(pub u32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DataIdx(pub u32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LocalIdx(pub u32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LabelIdx(pub u32);

impl Into<u32> for TypeIdx {
    fn into(self) -> u32 {
        self.0
    }
}
impl Parseable for TypeIdx {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(TypeIdx(u32::parse(reader)?))
    }
}
impl Into<u32> for FuncIdx {
    fn into(self) -> u32 {
        self.0
    }
}
impl Parseable for FuncIdx {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(FuncIdx(u32::parse(reader)?))
    }
}
impl Into<u32> for TableIdx {
    fn into(self) -> u32 {
        self.0
    }
}
impl Parseable for TableIdx {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(TableIdx(u32::parse(reader)?))
    }
}
impl Into<u32> for MemIdx {
    fn into(self) -> u32 {
        self.0
    }
}
impl Parseable for MemIdx {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(MemIdx(u32::parse(reader)?))
    }
}
impl Into<u32> for GlobalIdx {
    fn into(self) -> u32 {
        self.0
    }
}
impl Parseable for GlobalIdx {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(GlobalIdx(u32::parse(reader)?))
    }
}
impl Into<u32> for ElemIdx {
    fn into(self) -> u32 {
        self.0
    }
}
impl Parseable for ElemIdx {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(ElemIdx(u32::parse(reader)?))
    }
}
impl Into<u32> for DataIdx {
    fn into(self) -> u32 {
        self.0
    }
}
impl Parseable for DataIdx {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(DataIdx(u32::parse(reader)?))
    }
}
impl Into<u32> for LocalIdx {
    fn into(self) -> u32 {
        self.0
    }
}
impl Parseable for LocalIdx {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(LocalIdx(u32::parse(reader)?))
    }
}
impl Into<u32> for LabelIdx {
    fn into(self) -> u32 {
        self.0
    }
}
impl Parseable for LabelIdx {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(LabelIdx(u32::parse(reader)?))
    }
}

impl<T: Parseable> Parseable for Vec<T> {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Vec<T>> {
        let num = u32::from(Leb128::<u32>::parse(reader)?);
        let mut vec: Vec<T> = Vec::new();
        for _ in 0..num {
            let elem: T = T::parse(reader)?;
            vec.push(elem);
        }
        Ok(vec)
    }
}

impl Parseable for String {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<String> {
        let vec = Vec::<u8>::parse(reader)?;
        let string = String::from_utf8(vec)?;
        Ok(string)
    }
}

impl Parseable for u8 {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut buf: [u8; 1] = [0; 1];
        let n = reader.read(&mut buf[..]).unwrap();
        match n {
            1 => Ok(u8::from_le_bytes(buf)),
            n => Err(ParseError::wrong_num_bytes_read(Asked(1), Received(n))),
        }
    }
}

impl Parseable for u32 {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<u32> {
        let mut buf: [u8; 4] = [0; 4];
        let n = reader.read(&mut buf[..]).unwrap();
        match n {
            4 => Ok(u32::from_le_bytes(buf)),
            n => Err(ParseError::wrong_num_bytes_read(Asked(4), Received(n))),
        }
    }
}
