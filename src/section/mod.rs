pub mod code;
pub mod custom;
pub mod data_count;
pub mod data;
pub mod element;
pub mod export;
pub mod function;
pub mod global;
pub mod import;
pub mod memory;
pub mod start;
pub mod table;
pub mod r#type;

use crate::types::primitives::{TypeIdx, Size};

use std::fmt;

pub const CUSTOM_SECTION_ID: TypeIdx = 0;
pub const TYPE_SECTION_ID: TypeIdx = 1;
pub const IMPORT_SECTION_ID: TypeIdx = 2;
pub const FUNCTION_SECTION_ID: TypeIdx = 3;
pub const TABLE_SECTION_ID: TypeIdx = 4;
pub const MEMORY_SECTION_ID: TypeIdx = 5;
pub const GLOBAL_SECTION_ID: TypeIdx = 6;
pub const EXPORT_SECTION_ID: TypeIdx = 7;
pub const START_SECTION_ID: TypeIdx = 8;
pub const ELEMENT_SECTION_ID: TypeIdx = 9;
pub const CODE_SECTION_ID: TypeIdx = 10;
pub const DATA_SECTION_ID: TypeIdx = 11;
pub const DATA_COUNT_SECTION_ID: TypeIdx = 12;

pub trait Section: fmt::Display {
    fn section_type(&self) -> &str;
    fn size(&self) -> Size;
}

pub struct SectionParseError(String);