use std::io::{BufReader, Read};
use std::result::Result;

use crate::parseable::{ParseError, Parseable, Received};
use crate::section::code::CodeSec;
use crate::section::custom::{CustomSec, CustomSecParseError};
use crate::section::data::DataSec;
use crate::section::data_count::DataCountSec;
use crate::section::element::ElemSec;
use crate::section::export::ExportSec;
use crate::section::function::FunctionSec;
use crate::section::global::GlobalSec;
use crate::section::import::ImportSec;
use crate::section::memory::MemSec;
use crate::section::start::StartSec;
use crate::section::table::TableSec;
use crate::section::r#type::TypeSec;
use crate::section::{
    CODE_SECTION_ID, CUSTOM_SECTION_ID, DATA_COUNT_SECTION_ID, DATA_SECTION_ID, ELEMENT_SECTION_ID,
    EXPORT_SECTION_ID, FUNCTION_SECTION_ID, GLOBAL_SECTION_ID, IMPORT_SECTION_ID,
    MEMORY_SECTION_ID, START_SECTION_ID, Section, SectionParseError, TABLE_SECTION_ID,
    TYPE_SECTION_ID,
};
use crate::types::primitives::TypeIdx;

#[derive(Default)]
pub struct Module {
    pub version: u32,
    pub customsecs: Vec<CustomSec>,
    pub typesec: Option<TypeSec>,
    pub importsec: Option<ImportSec>,
    pub functionsec: Option<FunctionSec>,
    pub tablesec: Option<TableSec>,
    pub memsec: Option<MemSec>,
    pub globalsec: Option<GlobalSec>,
    pub exportsec: Option<ExportSec>,
    pub startsec: Option<StartSec>,
    pub elemsec: Option<ElemSec>,
    pub codesec: Option<CodeSec>,
    pub datasec: Option<DataSec>,
    pub datacountsec: Option<DataCountSec>,
}

pub enum ModuleParseError {
    Parse(ParseError),
    BadMagic([u8; 4]),
    InvalidVersion(u32),
    SectionParseError(SectionParseError),
}

impl From<ParseError> for ModuleParseError {
    fn from(value: ParseError) -> Self {
        ModuleParseError::Parse(value)
    }
}

impl From<SectionParseError> for ModuleParseError {
    fn from(value: SectionParseError) -> Self {
        ModuleParseError::SectionParseError(value)
    }
}

impl From<CustomSecParseError> for ModuleParseError {
    fn from(value: CustomSecParseError) -> Self {
        ModuleParseError::SectionParseError(value.into())
    }
}

impl Into<ParseError> for ModuleParseError {
    fn into(self) -> ParseError {
        match self {
            ModuleParseError::BadMagic(magic) => {
                ParseError::Other(format!("bad magic: {:#?}", magic))
            }
            ModuleParseError::InvalidVersion(version) => {
                ParseError::Other(format!("bad version: {}", version))
            }
            err => err.into(),
        }
    }
}

impl Module {
    fn parse_magic(reader: &mut BufReader<dyn Read>) -> Result<(), ModuleParseError> {
        let magic = u32::parse(reader)?;
        let bytes = magic.to_le_bytes();

        // `0x00 asm` in ASCII
        if bytes != [0, 97, 115, 109] {
            return Err(ModuleParseError::BadMagic(bytes));
        }

        Ok(())
    }

    fn parse_version(reader: &mut BufReader<dyn Read>) -> Result<u32, ModuleParseError> {
        match u32::parse(reader) {
            Err(e) => Err(ModuleParseError::Parse(e)),
            Ok(version) => Ok(version),
        }
    }

    pub fn sections(&self) -> Vec<&dyn Section> {
        let mut vec = Vec::<&dyn Section>::new();

        for customsec in &self.customsecs {
            vec.push(customsec);
        }
        if self.codesec.is_some() {
            vec.push(self.codesec.as_ref().unwrap());
        }
        if self.datacountsec.is_some() {
            vec.push(self.datacountsec.as_ref().unwrap());
        }
        if self.datasec.is_some() {
            vec.push(self.datasec.as_ref().unwrap());
        }
        if self.elemsec.is_some() {
            vec.push(self.elemsec.as_ref().unwrap());
        }
        if self.exportsec.is_some() {
            vec.push(self.exportsec.as_ref().unwrap());
        }
        if self.functionsec.is_some() {
            vec.push(self.functionsec.as_ref().unwrap());
        }
        if self.globalsec.is_some() {
            vec.push(self.globalsec.as_ref().unwrap());
        }
        if self.importsec.is_some() {
            vec.push(self.importsec.as_ref().unwrap());
        }
        if self.memsec.is_some() {
            vec.push(self.memsec.as_ref().unwrap());
        }
        if self.startsec.is_some() {
            vec.push(self.startsec.as_ref().unwrap());
        }
        if self.tablesec.is_some() {
            vec.push(self.tablesec.as_ref().unwrap());
        }
        if self.typesec.is_some() {
            vec.push(self.typesec.as_ref().unwrap());
        }

        vec
    }

    pub fn parse(
        reader: &mut std::io::BufReader<dyn std::io::Read>,
    ) -> Result<Module, ModuleParseError> {
        Self::parse_magic(reader)?;
        let version = Self::parse_version(reader)?;

        if version != 1 {
            return Err(ModuleParseError::InvalidVersion(version));
        }

        let mut module = Module {
            version: version,
            ..Default::default()
        };

        loop {
            let res = match u8::parse(reader) {
                Ok(n) => n,
                Err(ParseError::WrongNumBytesRead(_, Received(0))) => {
                    // We are out of bytes.
                    break;
                }
                Err(e) => return Err(ModuleParseError::Parse(e)),
            };
            let section_type = TypeIdx(u32::from(res));
            if section_type == CODE_SECTION_ID {
                module.codesec = Some(CodeSec::parse(reader)?);
            } else if section_type == CUSTOM_SECTION_ID {
                module.customsecs.push(CustomSec::parse(reader)?);
            } else if section_type == DATA_COUNT_SECTION_ID {
                module.datacountsec = Some(DataCountSec::parse(reader)?);
            } else if section_type == DATA_SECTION_ID {
                module.datasec = Some(DataSec::parse(reader)?);
            } else if section_type == ELEMENT_SECTION_ID {
                module.elemsec = Some(ElemSec::parse(reader)?);
            } else if section_type == EXPORT_SECTION_ID {
                module.exportsec = Some(ExportSec::parse(reader)?);
            } else if section_type == FUNCTION_SECTION_ID {
                module.functionsec = Some(FunctionSec::parse(reader)?);
            } else if section_type == GLOBAL_SECTION_ID {
                module.globalsec = Some(GlobalSec::parse(reader)?);
            } else if section_type == IMPORT_SECTION_ID {
                module.importsec = Some(ImportSec::parse(reader)?);
            } else if section_type == MEMORY_SECTION_ID {
                module.memsec = Some(MemSec::parse(reader)?);
            } else if section_type == START_SECTION_ID {
                module.startsec = Some(StartSec::parse(reader)?);
            } else if section_type == TABLE_SECTION_ID {
                module.tablesec = Some(TableSec::parse(reader)?);
            } else if section_type == TYPE_SECTION_ID {
                module.typesec = Some(TypeSec::parse(reader)?);
            } else {
                break;
            }
        }

        Ok(module)
    }
}
