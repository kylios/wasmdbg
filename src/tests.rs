use super::*;
use std::io::Cursor;

#[test]    
fn test_u32() {
    let bytes: [u8; 3] = [0xE5, 0x8E, 0x26];
    let mut reader = BufReader::new(Cursor::new(bytes));
    let result = u32::parse(&mut reader);
    assert!(result.is_ok());
    let val = result.expect("The parsed value");
    assert_eq!(val, 624485);
}

#[test]
fn test_i32() {
    let bytes: [u8; 3] = [0xc0, 0xbb, 0x78];
    let mut reader = BufReader::new(Cursor::new(bytes));
    let result = i32::parse(&mut reader);
    assert!(result.is_ok());
    let val = result.expect("The parsed value");
    assert_eq!(val, -123456);
}

#[test]
fn test_vec() {
    let bytes: [u8; 4] = [0x03, 0x01, 0x02, 0x03];
    let mut reader = BufReader::new(Cursor::new(bytes));
    let result: crate::parseable::Result<Vec<u32>> = Vec::parse(&mut reader);
    assert!(result.is_ok());
    let val = result.expect("The parsed value");
    assert_eq!(val, vec!(1, 2, 3));
}