use std::io::{BufReader, Read};
use std::fmt::Display;

use crate::parseable::{Parseable, Result, ParseError};
use crate::types::leb128::Leb128;

pub enum NumType {
    I32,
    I64,
    F32,
    F64
}

impl Display for NumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumType::I32 => write!(f, "i32"),
            NumType::I64 => write!(f, "i64"),
            NumType::F32 => write!(f, "f32"),
            NumType::F64 => write!(f, "F64")
        }
    }
}

pub enum VecType {
    V128
}

impl Display for VecType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VecType::V128 => write!(f, "v128")
        }
    }
}

pub enum RefType {
    Func,
    Extern
}

impl Parseable for RefType {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
        where
            Self: Sized {
        
        let mut buf: [u8; 1] = [0; 1];
        let n = reader.read(&mut buf)?;
        match n {
            1 => match u8::from_le_bytes(buf) {
                0x70 => Ok(RefType::Func),
                0x6f => Ok(RefType::Extern),
                _ => Err(ParseError::new("Value is not RefType".to_string()))
            },
            n => Err(ParseError::WrongNumBytesRead(1, n))
        }
    }
}

impl Display for RefType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RefType::Func => write!(f, "func"),
            RefType::Extern => write!(f, "extern")
        }
    }
}

pub enum ValType {
    Num(NumType),
    Vec(VecType),
    Ref(RefType)
}

pub type ResultType = Vec<ValType>;

impl Display for ValType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValType::Num(t) => write!(f, "numtype {}", t),
            ValType::Vec(t) => write!(f, "vectype {}", t),
            ValType::Ref(t) => write!(f, "reftype {}", t)
        }
    }
}

impl Parseable for ValType {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
        where
            Self: Sized {
        
        let mut buf: [u8; 1] = [0; 1];
        let n = reader.read(&mut buf)?;
        match n {
            1 => match u8::from_le_bytes(buf) {
                0x7f => Ok(ValType::Num(NumType::I32)),
                0x7e => Ok(ValType::Num(NumType::I64)),
                0x7d => Ok(ValType::Num(NumType::F32)),
                0x7c => Ok(ValType::Num(NumType::F64)),
                0x7b => Ok(ValType::Vec(VecType::V128)),
                0x70 => Ok(ValType::Ref(RefType::Func)),
                0x6f => Ok(ValType::Ref(RefType::Extern)),
                _ => Err(ParseError::Other("Value is not ValType".to_string()))
            },
            n => Err(ParseError::WrongNumBytesRead(1, n))
        }
    }
}

pub struct FuncType {
    rt1: ResultType,
    rt2: ResultType    
}

impl FuncType {
    fn parse_first_byte(reader: &mut BufReader<dyn Read>) -> Result<u8> {
        let mut buf: [u8; 1] = [0; 1];
        let n = reader.read(&mut buf)?;
        match n {
            1 => Ok(u8::from_le_bytes(buf)),
            n => Err(ParseError::WrongNumBytesRead(1, n))
        } 
    }
}

impl Display for FuncType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "* rt1: ")?;
        for rt in &self.rt1 {
            write!(f, "{}, ", rt)?;
        }
        writeln!(f)?;
        write!(f, "* rt2: ")?;
        for rt in &self.rt2 {
            write!(f, "{}, ", rt)?;
        }
        writeln!(f)?;

        Ok(())
    }
}

impl Parseable for FuncType {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
        where
            Self: Sized {
        
        // First byte is 0x60
        let first_byte = Self::parse_first_byte(reader)?;
        if first_byte != 0x60 {
            return Err(ParseError::Other("First byte of FuncType should be 0x60".to_string()));
        }
        
        let rt1 = ResultType::parse(reader)?;
        let rt2 = ResultType::parse(reader)?;
        
        let func = FuncType {
            rt1: rt1,
            rt2: rt2
        };
        
        Ok(func)
    }
}

pub struct Limits {
    min: u32,
    max: Option<u32>
}

impl Parseable for Limits {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
        where
            Self: Sized {
        
        let flag = u8::parse(reader)?;
        let min = u32::from(Leb128::<u32>::parse(reader)?);
        let mut max: Option<u32> = None;
        if flag == 0x1 {
            let max_val = u32::from(Leb128::<u32>::parse(reader)?);
            max = Some(max_val);
        }
        
        Ok(Limits {
            min,
            max
        })
    }
}

pub enum Mut {
    Val(u8)
}

pub const CONST: Mut = Mut::Val(0x0);
pub const VAR: Mut = Mut::Val(0x1);

impl Parseable for Mut {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
        where
            Self: Sized {
        
        let mut bytes: [u8; 1] = [0; 1];
        let n = reader.read(&mut bytes)?;
        match n {
            1 => match bytes[0] {
                0x0 => Ok(CONST),
                0x1 => Ok(VAR),
                _ => Err(ParseError::Other("Mut type must be 0x0 or 0x1".to_string()))
            },
            n => Err(ParseError::WrongNumBytesRead(1, n))
        }
    }
}