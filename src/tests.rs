use super::*;
use crate::types::leb128::Leb128;
use crate::parseable::Parseable;
use std::io::Cursor;

#[test]    
fn test_u32() {
    let bytes: [u8; 3] = [0xE5, 0x8E, 0x26];
    let mut reader = BufReader::new(Cursor::new(bytes));
    let result = Leb128::<u32>::parse(&mut reader);
    assert!(result.is_ok());
    let val = u32::from(result.expect("The parsed value"));
    assert_eq!(val, 624485);
}

#[test]
fn test_i32() {
    let bytes: [u8; 3] = [0xc0, 0xbb, 0x78];
    let mut reader = BufReader::new(Cursor::new(bytes));
    let result = Leb128::<i32>::parse(&mut reader);
    assert!(result.is_ok());
    let val = i32::from(result.expect("The parsed value"));
    assert_eq!(val, -123456);
}

#[test]
fn test_vec() {
    let bytes: [u8; 4] = [0x03, 0x01, 0x02, 0x03];
    let mut reader = BufReader::new(Cursor::new(bytes));
    let result: crate::parseable::Result<Vec<Leb128<u32>>> = Vec::parse(&mut reader);
    assert!(result.is_ok());
    let val = result.expect("The parsed value");
    let nums: Vec<u32> = val.iter().map(|v| u32::from(v)).collect();
    assert_eq!(nums, vec!(1, 2, 3));
}