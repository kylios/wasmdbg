use crate::types::{Size, TypeIdx};
use crate::parseable::Result;

use std::io::{Read, BufReader};

pub fn parse(reader: &mut BufReader<dyn Read>) -> Result<dyn Section> {
    let size = Size::parse(reader);
    let section_type = TypeIdx::parse(reader);
    
    
}

pub trait Section {
    fn parse(size: Size, reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized;
}

static CUSTOM_SECTION_ID: TypeIdx = 0;
static TYPE_SECTION_ID: TypeIdx = 1;
static IMPORT_SECTION_ID: TypeIdx = 2;
static FUNCTION_SECTION_ID: TypeIdx = 3;
static TABLE_SECTION_ID: TypeIdx = 4;
static MEMORY_SECTION_ID: TypeIdx = 5;
static GLOBAL_SECTION_ID: TypeIdx = 6;
static EXPORT_SECTION_ID: TypeIdx = 7;
static START_SECTION_ID: TypeIdx = 8;
static ELEMENT_SECTION_ID: TypeIdx = 9;
static CODE_SECTION_ID: TypeIdx = 10;
static DATA_SECTION_ID: TypeIdx = 11;
static DATA_COUNT_SECTION_ID: TypeIdx = 12;

pub struct TypeSection {

}

pub struct ImportSection {

}

pub struct FunctionSection {

}

pub struct MemorySection {

}

pub struct ExportSection {

}

pub struct CodeSection {

}

pub struct DataSection {

}

pub struct CustomSection {
    
}