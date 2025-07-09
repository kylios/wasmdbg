mod code;
mod custom;
mod data_count;
mod data;
mod element;
mod export;
mod function;
mod global;
mod import;
mod memory;
mod start;
mod table;
mod r#type;

use crate::types::{Leb128, Size, TypeIdx};
use crate::parseable::{Result, Parseable};

use std::io::{Read, BufReader};
use std::fmt;

const CUSTOM_SECTION_ID: TypeIdx = 0;
const TYPE_SECTION_ID: TypeIdx = 1;
const IMPORT_SECTION_ID: TypeIdx = 2;
const FUNCTION_SECTION_ID: TypeIdx = 3;
const TABLE_SECTION_ID: TypeIdx = 4;
const MEMORY_SECTION_ID: TypeIdx = 5;
const GLOBAL_SECTION_ID: TypeIdx = 6;
const EXPORT_SECTION_ID: TypeIdx = 7;
const START_SECTION_ID: TypeIdx = 8;
const ELEMENT_SECTION_ID: TypeIdx = 9;
const CODE_SECTION_ID: TypeIdx = 10;
const DATA_SECTION_ID: TypeIdx = 11;
const DATA_COUNT_SECTION_ID: TypeIdx = 12;

pub trait Section: fmt::Display {
    fn section_type(&self) -> &str;
    fn size(&self) -> Size;
}

pub fn parse(reader: &mut BufReader<dyn Read>) -> Result<Box<dyn Section>> {
    let section_type = u32::from(u8::parse(reader)?);
    let size = Leb128::<Size>::parse(reader)?.val;
    
    let section: Box<dyn Section> = match section_type {
        CUSTOM_SECTION_ID => custom::parse(size, reader),
        TYPE_SECTION_ID => r#type::parse(size, reader),
        IMPORT_SECTION_ID => import::parse(size, reader),
        FUNCTION_SECTION_ID => function::parse(size, reader),
        TABLE_SECTION_ID => table::parse(size, reader),
        GLOBAL_SECTION_ID => global::parse(size, reader),
        MEMORY_SECTION_ID => memory::parse(size, reader),
        EXPORT_SECTION_ID => export::parse(size, reader),
        START_SECTION_ID => start::parse(size, reader),
        ELEMENT_SECTION_ID => element::parse(size, reader),
        CODE_SECTION_ID => code::parse(size, reader),
        DATA_SECTION_ID => data::parse(size, reader),
        DATA_COUNT_SECTION_ID => data_count::parse(size, reader),
        _ => panic!("Invalid Section Type")
    }?;
    
    Ok(section)
}
