use crate::parseable::Parseable;

use std::io::{BufReader, Read};

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

impl<T: Parseable> Parseable for Vec<T> {
    fn parse(reader: &mut BufReader<dyn Read>) -> Self {
        let num: Size = Size::parse(reader);
        let mut vec: Vec<T> = Vec::new();
        for _ in 0..num {
            let elem: T = T::parse(reader);
            vec.push(elem);
        }
        vec
    }
}

impl Parseable for u32 {
    fn parse(reader: &mut BufReader<dyn Read>) -> Self {
        let mut num: u32 = 0;
        let mut shift: u32 = 0;
        let mut buf: [u8; 1] = [0];
        let mut val: u8;
        loop {
            reader.read(&mut buf[..]).unwrap();
            val = u8::from_le_bytes(buf);
            num += u32::from(val & 127) << shift;
            if val & 128 == 0 {
                break ()
            }
            shift += 7;
        }
        num
    }
}

impl Parseable for i32 {
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
    fn parse(reader: &mut BufReader<dyn Read>) -> Self {
        let mut num: i32 = 0;
        let mut shift: u32 = 0;
        let mut buf: [u8; 1] = [0];
        let mut val: u8;

        loop {
            reader.read(&mut buf[..]).unwrap();
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
        num
    }
}

impl Parseable for u64 {
    fn parse(reader: &mut BufReader<dyn Read>) -> Self {
        0
    }
}

impl Parseable for i64 {
    fn parse(reader: &mut BufReader<dyn Read>) -> Self {
        0
    }
}