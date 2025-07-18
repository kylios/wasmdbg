use super::*;
use crate::parseable::Parseable;
use crate::types::leb128::Leb128;
use std::io::{BufReader, Cursor};

#[test]
fn test_u8() {
    let bytes: [u8; 2] = [0x01, 0x02];
    let mut reader = BufReader::new(Cursor::new(bytes));
    let result = u8::parse(&mut reader);
    assert!(result.is_ok());
    let val = result.expect("The parsed value");
    assert_eq!(val, 1);

    let result = u8::parse(&mut reader);
    assert!(result.is_ok());
    let val = result.expect("The parsed value");
    assert_eq!(val, 2);

    let result = u8::parse(&mut reader);
    assert!(result.is_err());
}

#[test]
fn test_u32() {
    let bytes: [u8; 4] = [0xEF, 0xBE, 0xAD, 0xDE];
    let mut reader = BufReader::new(Cursor::new(bytes));
    let result = u32::parse(&mut reader);
    assert!(result.is_ok());
    let val = result.expect("The parsed value");
    assert_eq!(val, 3735928559);

    // Give too few bytes
    let bytes: [u8; 2] = [0xEF, 0xBE];
    let mut reader = BufReader::new(Cursor::new(bytes));
    let result = u32::parse(&mut reader);
    assert!(result.is_err());
}

#[test]
fn test_leb128_u32() {
    let bytes: [u8; 3] = [0xE5, 0x8E, 0x26];
    let mut reader = BufReader::new(Cursor::new(bytes));
    let result = Leb128::<u32>::parse(&mut reader);
    assert!(result.is_ok());
    let val = u32::from(result.expect("The parsed value"));
    assert_eq!(val, 624485);
}

#[test]
fn test_leb128_i32() {
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
    let result = Vec::parse(&mut reader);
    assert!(result.is_ok());
    let val = result.expect("The parsed value");
    let nums: Vec<u32> = val.iter().map(|v| u32::from(v)).collect();
    assert_eq!(nums, vec!(1, 2, 3));
}

#[test]
fn test_string() {
    let bytes: [u8; 4] = [0x03, 0x61, 0x73, 0x6d];
    let mut reader = BufReader::new(Cursor::new(bytes));
    let result = String::parse(&mut reader);
    assert!(result.is_ok());
    let val = result.expect("The parsed value");
    assert_eq!(val, "asm");
}
