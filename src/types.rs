use crate::parseable::{Parseable, Result, ParseError};

use std::io::{BufReader, Read};
use std::fmt::Display;

// TODO: split up this file into multiple

pub type Size = u32;
pub type TypeIdx = u32;
pub type FuncIdx = u32;
pub type TableIdx = u32;
pub type MemIdx = u32;
pub type GlobalIdx = u32;
pub type ElemIdx = u32;
pub type DataIdx = u32;
pub type LocalIdx = u32;
pub type LabelIdx = u32;

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
                _ => Err(ParseError::new("Value is not RefType"))
            },
            _ => Err(ParseError::new("Should have read 1 byte"))
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
                _ => Err(ParseError::new("Value is not ValType"))
            },
            _ => Err(ParseError::new("Should have read 1 byte"))
        }
    }
}

impl<T: Parseable> Parseable for Vec<T> {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Vec<T>> {
        let num = Leb128::<Size>::parse(reader)?.val;
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
            Self: Sized {
        let mut buf: [u8; 1] = [0; 1];
        let n = reader.read(&mut buf[..]).unwrap();
        match n {
            1 => Ok(u8::from_le_bytes(buf)),
            _ => Err(ParseError::new("Should have read 1 byte"))
        }
        
    }
}

impl Parseable for u32 {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<u32> {
        let mut buf: [u8; 4] = [0; 4];
        let n = reader.read(&mut buf[..]).unwrap();
        match n {
            4 => Ok(u32::from_le_bytes(buf)),
            _ => Err(ParseError::new("Should have read 4 bytes"))
        }
    }
}

pub struct Leb128<T> {
    pub val: T
}

impl Parseable for Leb128<u32> {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Leb128<u32>> {
        let mut num: u32 = 0;
        let mut shift: u32 = 0;
        let mut buf: [u8; 1] = [0];
        let mut val: u8;
        loop {
            let n = reader.read(&mut buf[..]).unwrap();
            if n != 1 {
                return Err(ParseError::new("Should have read 1 byte"));
            }
            val = u8::from_le_bytes(buf);
            num += u32::from(val & 127) << shift;
            if val & 128 == 0 {
                break ()
            }
            shift += 7;
        }
        Ok(Leb128 {
            val: num
        })
    }
}

impl From<Leb128<u32>> for u32  {
    fn from(value: Leb128<u32>) -> Self {
        value.val
    }
}

// TODO: can we more elegantly convert a 
// vector of LEB types to their natural types?
/* impl<T> From<Vec<Leb128<T>>> for Vec<T> {
    fn from(value: Vec<Leb128<T>>) -> Self {
        value.iter().map(|v| v.val).collect()
    }
} */

impl Parseable for Leb128<i32> {
    //    MSB ------------------ LSB
    //         11110001001000000  Binary encoding of 123456
    //     000011110001001000000  As a 21-bit number
    //     111100001110110111111  Negating all bits (ones' complement)
    //     111100001110111000000  Adding one (two's complement)
    // 1111000  0111011  1000000  Split into 7-bit groups
    //01111000 10111011 11000000  Add high 1 bits on all but last (most significant) group to form bytes
    //    0x78     0xBB     0xC0  In hexadecimal
    //
    //→ 0xC0 0xBB 0x78            Output stream (LSB to MSB)
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Leb128<i32>> {
        let mut num: i32 = 0;
        let mut shift: u32 = 0;
        let mut buf: [u8; 1] = [0];
        let mut val: u8;

        loop {
            let n = reader.read(&mut buf[..]).unwrap();
            if n != 1 {
                return Err(ParseError::new("Should have read 1 byte"));
            }
            val = u8::from_le_bytes(buf);
            num += i32::from(val & 127) << shift;
            shift += 7;
            if val & 128 == 0 {
                break ()
            }
        }
        
        let i32_size: u32 = size_of::<i32>().try_into().unwrap();
        if shift < (i32_size * 8) && val & 0x40 != 0 {
            num |= -(1 << shift);
        }
        Ok(Leb128 {
            val: num
        })
    }
}


impl From<Leb128<i32>> for i32 {
    fn from(value: Leb128<i32>) -> Self {
        value.val
    }
}

impl Parseable for Leb128<u64> {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Leb128<u64>> {
        Ok(Leb128 {
            val: 0
        })
    }
}

impl From<Leb128<u64>> for u64 {
    fn from(value: Leb128<u64>) -> Self {
        value.val
    }
}

impl Parseable for Leb128<i64> {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Leb128<i64>> {
        Ok(Leb128 {
            val: 0
        })
    }
}

impl From<Leb128<i64>> for i64 {
    fn from(value: Leb128<i64>) -> Self {
        value.val
    }
}

pub struct FuncType {
    rt1: ResultType,
    rt2: ResultType
}

impl FuncType {
    fn parse_first_byte(reader: &mut BufReader<dyn Read>) -> Result<u8> {
        let mut buf: [u8; 1] = [0; 1];
        let n = reader.read(&mut buf);
        match n {
            Ok(1) => Ok(u8::from_le_bytes(buf)),
            _ => Err(ParseError::new("Should have read 1 byte"))
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
            return Err(ParseError::new("First byte of FuncType should be 0x60"));
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
        let min = Leb128::<u32>::parse(reader)?.val;
        let mut max: Option<u32> = None;
        if flag == 0x1 {
            let max_val = Leb128::<u32>::parse(reader)?.val;
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
                _ => Err(ParseError::new("Mut type must be 0x0 or 0x1"))
            },
            _ => Err(ParseError::new("Should have read 1 byte"))
        }
    }
}