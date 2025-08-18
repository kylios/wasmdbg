pub mod numeric;
pub mod vector;
pub mod reference;
pub mod parametric;
pub mod variable;
pub mod table;
pub mod memory;
pub mod control;

use std::fmt::Display;
use std::io::{BufReader, Read};

use crate::parseable::{ParseError, Parseable, ReadSeek};
use crate::types::leb128::Leb128;
use crate::types::num_type::{NumType, IType, FType};
use crate::types::val_type::ValType;
use crate::types::primitives::{LabelIdx, TypeIdx, FuncIdx, TableIdx, DataIdx, LaneIdx};
use crate::types::mem_arg::MemArg;
use crate::instructions::instr::numeric::NumericInstr;
use crate::instructions::instr::vector::VectorInstr;
use crate::instructions::instr::reference::ReferenceInstr;
use crate::instructions::instr::parametric::ParametricInstr;
use crate::instructions::instr::variable::VariableInstr;
use crate::instructions::instr::table::TableInstr;
use crate::instructions::instr::memory::MemoryInstr;
use crate::instructions::instr::control::ControlInstr;

pub enum Num {
    I32(Leb128<i32>),
    I64(Leb128<i64>),
    //F() // TODO: we don't have a construct for parsing floating point numbers yet
}

/*
 * blocktype := typeidx | valtype
 *
 * Note:
 * Value types can occur in contexts where type indices are also allowed,
 * such as in the case of block types. Thus, the binary format for types
 * corresponds to the signed LEB128 encoding of small negative values, so
 * that they can coexist with (positive) type indices in the future.
 * This affects the parsing of block types, as we will always attempt to
 * parse the `valtype` first, and if it fails, we will backtrack and try to
 * parse a `typeidx`.
 */
#[derive(Debug, PartialEq)]
pub enum BlockType {
    TypeIdx(TypeIdx),
    ValType(ValType)
}

impl Parseable for BlockType {
    fn parse(reader: &mut BufReader<dyn ReadSeek>) -> Result<BlockType, ParseError>
        where
            Self: Sized {

        let res = ValType::parse(reader);
        match res {
            Ok(v) => Ok(BlockType::ValType(v)),
            Err(_) => {
                // If the `ValType` failed to parse, then we need to backtrack
                // the parsing and try to parse a `TypeIdx`
                reader.seek_relative(-1 * ValType::parse_len() as i64)?;
                let type_idx = TypeIdx::parse(reader)?;
                Ok(BlockType::TypeIdx(type_idx))
            }
        }
    }
}

impl Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockType::TypeIdx(t) => write!(f, "{}", t),
            BlockType::ValType(t) => write!(f, "{}", t)
        }
    }
}

#[cfg(test)]
mod block_type_tests {
    use super::*;
    use crate::types::ref_type::RefType;
    use crate::types::vec_type::VecType;
    use crate::types::num_type::NumType;
    use std::io::Cursor;

    #[test]
    fn test_parse_block_type_ok() {
        // Test parsing all `valtype` types, then test parsing
        // the value as a `typeidx`. When parsing `typeidx` values
        // that fall within the range `0x6F` - `0x7F` (the range of
        // `valtype` types), the `typeidx` should be represented as
        // two bytes: `0xEF 0x00` - `0xFF 0x00` (decimal values 111 - 127).
        let bytes: [u8; 11] = [
            0x6F,
            0x70,
            0x7B,
            0x7C,
            0x7D,
            0x7E,
            0x7F,
            0xEF, 0x00,
            0xFF, 0x00,
        ];
        let mut reader = BufReader::new(Cursor::new(bytes));
        let result = BlockType::parse(&mut reader);
        assert!(result.is_ok());
        let val = result.expect("The parsed value");
        assert_eq!(val, BlockType::ValType(ValType::Ref(RefType::Extern)));

        let result = BlockType::parse(&mut reader);
        assert!(result.is_ok());
        let val = result.expect("The parsed value");
        assert_eq!(val, BlockType::ValType(ValType::Ref(RefType::Func)));

        let result = BlockType::parse(&mut reader);
        assert!(result.is_ok());
        let val = result.expect("The parsed value");
        assert_eq!(val, BlockType::ValType(ValType::Vec(VecType::V128)));

        let result = BlockType::parse(&mut reader);
        assert!(result.is_ok());
        let val = result.expect("The parsed value");
        assert_eq!(val, BlockType::ValType(ValType::Num(NumType::F(FType::F64))));

        let result = BlockType::parse(&mut reader);
        assert!(result.is_ok());
        let val = result.expect("The parsed value");
        assert_eq!(val, BlockType::ValType(ValType::Num(NumType::F(FType::F32))));

        let result = BlockType::parse(&mut reader);
        assert!(result.is_ok());
        let val = result.expect("The parsed value");
        assert_eq!(val, BlockType::ValType(ValType::Num(NumType::I(IType::I64))));

        let result = BlockType::parse(&mut reader);
        assert!(result.is_ok());
        let val = result.expect("The parsed value");
        assert_eq!(val, BlockType::ValType(ValType::Num(NumType::I(IType::I32))));

        let result = Leb128::<u32>::parse(&mut reader);
        assert!(result.is_ok());
        let val = u32::from(result.expect("The parsed value"));
        assert_eq!(val, 111);

        let result = Leb128::<u32>::parse(&mut reader);
        assert!(result.is_ok());
        let val = u32::from(result.expect("The parsed value"));
        assert_eq!(val, 127);
    }
}

/*
 * TODO: read this whole page: https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr
 */
pub enum Instr {
    Numeric(NumericInstr),
    Vector(VectorInstr),
    Reference(ReferenceInstr),
    Parametric(ParametricInstr),
    Variable(VariableInstr),
    Table(TableInstr),
    Memory(MemoryInstr),
    Control(ControlInstr),
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::Numeric(instr) => write!(f, "{}", instr),
            Instr::Vector(instr) => write!(f, "{}", instr),
            Instr::Reference(instr) => write!(f, "{}", instr),
            Instr::Parametric(instr) => write!(f, "{}", instr),
            Instr::Variable(instr) => write!(f, "{}", instr),
            Instr::Table(instr) => write!(f, "{}", instr),
            Instr::Memory(instr) => write!(f, "{}", instr),
            Instr::Control(instr) => write!(f, "{}", instr),
        }
    }
}

struct Asked(usize);
struct Received(usize);

pub enum InstrParseErr {
    WrongNumBytesRead(Asked, Received),
    InvalidInstr(u8),
    IoError(std::io::Error),
    ParseError(ParseError)
}

impl From<std::io::Error> for InstrParseErr {
    fn from(err: std::io::Error) -> Self {
        InstrParseErr::IoError(err)
    }
}

impl From<ParseError> for InstrParseErr {
    fn from(err: ParseError) -> Self {
        InstrParseErr::ParseError(err)
    }
}

// TODO: can we implement Parseable for this type? The match statement above is non-exhaustive,
// so we will need to return some type that indicates a byte is not a NumericInstr. Perhaps
// the parsing should occur at a higher level where the matching can be more exhaustive, and
// the correct instruction returned, whether it's a NumericInstr or something else.
impl Instr {
    pub fn parse(reader: &mut BufReader<dyn ReadSeek>) -> Result<Instr, InstrParseErr>
        where
            Self: Sized {
        let mut buf: [u8; 1] = [0];
        let n = reader.read(&mut buf)?;
        match n {
            1 => Instr::from(u8::from_le_bytes(buf), reader),
            n => Err(InstrParseErr::WrongNumBytesRead(Asked(1), Received(n)))
        }
    }

    fn from(byte: u8, reader: &mut BufReader<dyn ReadSeek>) -> Result<Instr, InstrParseErr> {
        match byte {
            0x00 => Ok(Instr::Control(ControlInstr::Unreachable)),
            0x01 => Ok(Instr::Control(ControlInstr::Nop)),
            0x02 => Ok(Instr::Control(ControlInstr::Block(BlockType::parse(reader)?))),
            0x03 => Ok(Instr::Control(ControlInstr::Loop(BlockType::parse(reader)?))),
            0x04 => Ok(Instr::Control(ControlInstr::If(BlockType::parse(reader)?))),
            0x05 => Ok(Instr::Control(ControlInstr::Else)),
            // 0x06 - 0x0A reserved
            0x0B => Ok(Instr::Control(ControlInstr::End)),
            0x0C => Ok(Instr::Control(ControlInstr::Br(LabelIdx::parse(reader)?))),
            0x0D => Ok(Instr::Control(ControlInstr::BrIf(LabelIdx::parse(reader)?))),
            0x0E => Ok(Instr::Control(ControlInstr::BrTable(Vec::<LabelIdx>::parse(reader)?, LabelIdx::parse(reader)?))),
            0x0F => Ok(Instr::Control(ControlInstr::Return)),
            0x10 => Ok(Instr::Control(ControlInstr::Call(FuncIdx::parse(reader)?))),
            0x11 => Ok(Instr::Control(ControlInstr::CallIndirect(TableIdx::parse(reader)?, TypeIdx::parse(reader)?))),
            // 0x12 - 0x19 reserved

            0x1A => Ok(Instr::Parametric(ParametricInstr::Drop)),
            0x1B => Ok(Instr::Parametric(ParametricInstr::Select(None))),
            0x1C => Ok(Instr::Parametric(ParametricInstr::Select(Some(ValType::parse(reader)?)))),

            // Memory Instructions
            0x28 => Ok(Instr::Memory(MemoryInstr::I32Load(MemArg::parse(reader)?))),
            0x29 => Ok(Instr::Memory(MemoryInstr::I64Load(MemArg::parse(reader)?))),
            0x2A => Ok(Instr::Memory(MemoryInstr::F32Load(MemArg::parse(reader)?))),
            0x2B => Ok(Instr::Memory(MemoryInstr::F64Load(MemArg::parse(reader)?))),
            0x2C => Ok(Instr::Memory(MemoryInstr::I32Load8S(MemArg::parse(reader)?))),
            0x2D => Ok(Instr::Memory(MemoryInstr::I32Load8U(MemArg::parse(reader)?))),
            0x2E => Ok(Instr::Memory(MemoryInstr::I32Load16S(MemArg::parse(reader)?))),
            0x2F => Ok(Instr::Memory(MemoryInstr::I32Load16U(MemArg::parse(reader)?))),
            0x30 => Ok(Instr::Memory(MemoryInstr::I64Load8S(MemArg::parse(reader)?))),
            0x31 => Ok(Instr::Memory(MemoryInstr::I64Load8U(MemArg::parse(reader)?))),
            0x32 => Ok(Instr::Memory(MemoryInstr::I64Load16S(MemArg::parse(reader)?))),
            0x33 => Ok(Instr::Memory(MemoryInstr::I64Load16U(MemArg::parse(reader)?))),
            0x34 => Ok(Instr::Memory(MemoryInstr::I64Load32S(MemArg::parse(reader)?))),
            0x35 => Ok(Instr::Memory(MemoryInstr::I64Load32U(MemArg::parse(reader)?))),
            0x36 => Ok(Instr::Memory(MemoryInstr::I32Store(MemArg::parse(reader)?))),
            0x37 => Ok(Instr::Memory(MemoryInstr::I64Store(MemArg::parse(reader)?))),
            0x38 => Ok(Instr::Memory(MemoryInstr::F32Store(MemArg::parse(reader)?))),
            0x39 => Ok(Instr::Memory(MemoryInstr::F64Store(MemArg::parse(reader)?))),
            0x3A => Ok(Instr::Memory(MemoryInstr::I32Store8(MemArg::parse(reader)?))),
            0x3B => Ok(Instr::Memory(MemoryInstr::I32Store16(MemArg::parse(reader)?))),
            0x3C => Ok(Instr::Memory(MemoryInstr::I64Store8(MemArg::parse(reader)?))),
            0x3D => Ok(Instr::Memory(MemoryInstr::I64Store16(MemArg::parse(reader)?))),
            0x3E => Ok(Instr::Memory(MemoryInstr::I64Store32(MemArg::parse(reader)?))),
            0x3F => Ok(Instr::Memory(MemoryInstr::MemorySize)),
            0x40 => Ok(Instr::Memory(MemoryInstr::MemoryGrow)),

            // Numeric Instructions
            0x41 => Ok(Instr::Numeric(NumericInstr::Const(Num::I32(Leb128::<i32>::parse(reader)?)))),
            0x42 => Ok(Instr::Numeric(NumericInstr::Const(Num::I64(Leb128::<i64>::parse(reader)?)))),
            // 0x43 => Ok(Instr::Numeric(NumericInstr::Const(NumType::F(FType::F32)))),
            // 0x44 => Ok(Instr::Numeric(NumericInstr::Const(NumType::F(FType::F64)))),
            0x45 => Ok(Instr::Numeric(NumericInstr::Eqz(IType::I32))),
            0x46 => Ok(Instr::Numeric(NumericInstr::Eq(NumType::I(IType::I32)))),
            0x47 => Ok(Instr::Numeric(NumericInstr::Ne(NumType::I(IType::I32)))),
            0x48 => Ok(Instr::Numeric(NumericInstr::LtS(IType::I32))),
            0x49 => Ok(Instr::Numeric(NumericInstr::LtU(IType::I32))),
            0x4a => Ok(Instr::Numeric(NumericInstr::GtS(IType::I32))),
            0x4b => Ok(Instr::Numeric(NumericInstr::GtU(IType::I32))),
            0x4c => Ok(Instr::Numeric(NumericInstr::LeS(IType::I32))),
            0x4d => Ok(Instr::Numeric(NumericInstr::LeU(IType::I32))),
            0x4e => Ok(Instr::Numeric(NumericInstr::GeS(IType::I32))),
            0x4f => Ok(Instr::Numeric(NumericInstr::GeU(IType::I32))),
            0x50 => Ok(Instr::Numeric(NumericInstr::Eqz(IType::I64))),
            0x51 => Ok(Instr::Numeric(NumericInstr::Eq(NumType::I(IType::I64)))),
            0x52 => Ok(Instr::Numeric(NumericInstr::Ne(NumType::I(IType::I64)))),
            0x53 => Ok(Instr::Numeric(NumericInstr::LtS(IType::I64))),
            0x54 => Ok(Instr::Numeric(NumericInstr::LtU(IType::I64))),
            0x55 => Ok(Instr::Numeric(NumericInstr::GtS(IType::I64))),
            0x56 => Ok(Instr::Numeric(NumericInstr::GtU(IType::I64))),
            0x57 => Ok(Instr::Numeric(NumericInstr::LeS(IType::I64))),
            0x58 => Ok(Instr::Numeric(NumericInstr::LeU(IType::I64))),
            0x59 => Ok(Instr::Numeric(NumericInstr::GeS(IType::I64))),
            0x5a => Ok(Instr::Numeric(NumericInstr::GeU(IType::I64))),
            0x5b => Ok(Instr::Numeric(NumericInstr::Eq(NumType::F(FType::F32)))),
            0x5c => Ok(Instr::Numeric(NumericInstr::Ne(NumType::F(FType::F32)))),
            0x5d => Ok(Instr::Numeric(NumericInstr::Lt(FType::F32))),
            0x5e => Ok(Instr::Numeric(NumericInstr::Gt(FType::F32))),
            0x5f => Ok(Instr::Numeric(NumericInstr::Le(FType::F32))),
            0x60 => Ok(Instr::Numeric(NumericInstr::Ge(FType::F32))),
            0x61 => Ok(Instr::Numeric(NumericInstr::Eq(NumType::F(FType::F64)))),
            0x62 => Ok(Instr::Numeric(NumericInstr::Ne(NumType::F(FType::F64)))),
            0x63 => Ok(Instr::Numeric(NumericInstr::Lt(FType::F64))),
            0x64 => Ok(Instr::Numeric(NumericInstr::Gt(FType::F64))),
            0x65 => Ok(Instr::Numeric(NumericInstr::Le(FType::F64))),
            0x66 => Ok(Instr::Numeric(NumericInstr::Ge(FType::F64))),
            0x67 => Ok(Instr::Numeric(NumericInstr::Clz(IType::I32))),
            0x68 => Ok(Instr::Numeric(NumericInstr::Ctz(IType::I32))),
            0x69 => Ok(Instr::Numeric(NumericInstr::Popcnt(IType::I32))),
            0x6a => Ok(Instr::Numeric(NumericInstr::Add(NumType::I(IType::I32)))),
            0x6b => Ok(Instr::Numeric(NumericInstr::Sub(NumType::I(IType::I32)))),
            0x6c => Ok(Instr::Numeric(NumericInstr::Mul(NumType::I(IType::I32)))),
            0x6d => Ok(Instr::Numeric(NumericInstr::DivS(IType::I32))),
            0x6e => Ok(Instr::Numeric(NumericInstr::DivU(IType::I32))),
            0x6f => Ok(Instr::Numeric(NumericInstr::RemS(IType::I32))),
            0x70 => Ok(Instr::Numeric(NumericInstr::RemU(IType::I32))),
            0x71 => Ok(Instr::Numeric(NumericInstr::And(IType::I32))),
            0x72 => Ok(Instr::Numeric(NumericInstr::Or(IType::I32))),
            0x73 => Ok(Instr::Numeric(NumericInstr::Xor(IType::I32))),
            0x74 => Ok(Instr::Numeric(NumericInstr::Shl(IType::I32))),
            0x75 => Ok(Instr::Numeric(NumericInstr::ShrS(IType::I32))),
            0x76 => Ok(Instr::Numeric(NumericInstr::ShrU(IType::I32))),
            0x77 => Ok(Instr::Numeric(NumericInstr::Rotl(IType::I32))),
            0x78 => Ok(Instr::Numeric(NumericInstr::Rotr(IType::I32))),
            0x79 => Ok(Instr::Numeric(NumericInstr::Clz(IType::I64))),
            0x7a => Ok(Instr::Numeric(NumericInstr::Ctz(IType::I64))),
            0x7b => Ok(Instr::Numeric(NumericInstr::Popcnt(IType::I64))),
            0x7c => Ok(Instr::Numeric(NumericInstr::Add(NumType::I(IType::I64)))),
            0x7d => Ok(Instr::Numeric(NumericInstr::Sub(NumType::I(IType::I64)))),
            0x7e => Ok(Instr::Numeric(NumericInstr::Mul(NumType::I(IType::I64)))),
            0x7f => Ok(Instr::Numeric(NumericInstr::DivS(IType::I64))),
            0x80 => Ok(Instr::Numeric(NumericInstr::DivU(IType::I64))),
            0x81 => Ok(Instr::Numeric(NumericInstr::RemS(IType::I64))),
            0x82 => Ok(Instr::Numeric(NumericInstr::RemU(IType::I64))),
            0x83 => Ok(Instr::Numeric(NumericInstr::And(IType::I64))),
            0x84 => Ok(Instr::Numeric(NumericInstr::Or(IType::I64))),
            0x85 => Ok(Instr::Numeric(NumericInstr::Xor(IType::I64))),
            0x86 => Ok(Instr::Numeric(NumericInstr::Shl(IType::I64))),
            0x87 => Ok(Instr::Numeric(NumericInstr::ShrS(IType::I64))),
            0x88 => Ok(Instr::Numeric(NumericInstr::ShrU(IType::I64))),
            0x89 => Ok(Instr::Numeric(NumericInstr::Rotl(IType::I64))),
            0x8a => Ok(Instr::Numeric(NumericInstr::Rotr(IType::I64))),
            0x8b => Ok(Instr::Numeric(NumericInstr::Abs(FType::F32))),
            0x8c => Ok(Instr::Numeric(NumericInstr::Neg(FType::F32))),
            0x8d => Ok(Instr::Numeric(NumericInstr::Ceil(FType::F32))),
            0x8e => Ok(Instr::Numeric(NumericInstr::Floor(FType::F32))),
            0x8f => Ok(Instr::Numeric(NumericInstr::Trunc(FType::F32))),
            0x90 => Ok(Instr::Numeric(NumericInstr::Nearest(FType::F32))),
            0x91 => Ok(Instr::Numeric(NumericInstr::Sqrt(FType::F32))),
            0x92 => Ok(Instr::Numeric(NumericInstr::Add(NumType::F(FType::F32)))),
            0x93 => Ok(Instr::Numeric(NumericInstr::Sub(NumType::F(FType::F32)))),
            0x94 => Ok(Instr::Numeric(NumericInstr::Mul(NumType::F(FType::F32)))),
            0x95 => Ok(Instr::Numeric(NumericInstr::Div(FType::F32))),
            0x96 => Ok(Instr::Numeric(NumericInstr::Min(FType::F32))),
            0x97 => Ok(Instr::Numeric(NumericInstr::Max(FType::F32))),
            0x98 => Ok(Instr::Numeric(NumericInstr::Copysign(FType::F32))),
            0x99 => Ok(Instr::Numeric(NumericInstr::Abs(FType::F64))),
            0x9a => Ok(Instr::Numeric(NumericInstr::Neg(FType::F64))),
            0x9b => Ok(Instr::Numeric(NumericInstr::Ceil(FType::F64))),
            0x9c => Ok(Instr::Numeric(NumericInstr::Floor(FType::F64))),
            0x9d => Ok(Instr::Numeric(NumericInstr::Trunc(FType::F64))),
            0x9e => Ok(Instr::Numeric(NumericInstr::Nearest(FType::F64))),
            0x9f => Ok(Instr::Numeric(NumericInstr::Sqrt(FType::F64))),
            0xa0 => Ok(Instr::Numeric(NumericInstr::Add(NumType::F(FType::F64)))),
            0xa1 => Ok(Instr::Numeric(NumericInstr::Sub(NumType::F(FType::F64)))),
            0xa2 => Ok(Instr::Numeric(NumericInstr::Mul(NumType::F(FType::F64)))),
            0xa3 => Ok(Instr::Numeric(NumericInstr::Div(FType::F64))),
            0xa4 => Ok(Instr::Numeric(NumericInstr::Min(FType::F64))),
            0xa5 => Ok(Instr::Numeric(NumericInstr::Max(FType::F64))),
            0xa6 => Ok(Instr::Numeric(NumericInstr::Copysign(FType::F64))),
            0xa7 => Ok(Instr::Numeric(NumericInstr::I32WrapI64)),
            0xa8 => Ok(Instr::Numeric(NumericInstr::ITruncS(IType::I32, FType::F32))),
            0xa9 => Ok(Instr::Numeric(NumericInstr::ITruncU(IType::I32, FType::F32))),
            0xaa => Ok(Instr::Numeric(NumericInstr::ITruncS(IType::I32, FType::F64))),
            0xab => Ok(Instr::Numeric(NumericInstr::ITruncU(IType::I32, FType::F64))),
            0xac => Ok(Instr::Numeric(NumericInstr::I64ExtendI32S)),
            0xad => Ok(Instr::Numeric(NumericInstr::I64ExtendI32U)),
            0xae => Ok(Instr::Numeric(NumericInstr::ITruncS(IType::I64, FType::F32))),
            0xaf => Ok(Instr::Numeric(NumericInstr::ITruncU(IType::I64, FType::F32))),
            0xb0 => Ok(Instr::Numeric(NumericInstr::ITruncS(IType::I64, FType::F64))),
            0xb1 => Ok(Instr::Numeric(NumericInstr::ITruncU(IType::I64, FType::F64))),
            0xb2 => Ok(Instr::Numeric(NumericInstr::FConvertIS(FType::F32, IType::I32))),
            0xb3 => Ok(Instr::Numeric(NumericInstr::FConvertIU(FType::F32, IType::I32))),
            0xb4 => Ok(Instr::Numeric(NumericInstr::FConvertIS(FType::F32, IType::I64))),
            0xb5 => Ok(Instr::Numeric(NumericInstr::FConvertIU(FType::F32, IType::I64))),
            0xb6 => Ok(Instr::Numeric(NumericInstr::F32DemoteF64)),
            0xb7 => Ok(Instr::Numeric(NumericInstr::FConvertIS(FType::F64, IType::I32))),
            0xb8 => Ok(Instr::Numeric(NumericInstr::FConvertIU(FType::F64, IType::I32))),
            0xb9 => Ok(Instr::Numeric(NumericInstr::FConvertIS(FType::F64, IType::I64))),
            0xba => Ok(Instr::Numeric(NumericInstr::FConvertIU(FType::F64, IType::I64))),
            0xbb => Ok(Instr::Numeric(NumericInstr::F64PromoteF32)),
            0xbc => Ok(Instr::Numeric(NumericInstr::IReinterpretF(IType::I32, FType::F32))),
            0xbd => Ok(Instr::Numeric(NumericInstr::IReinterpretF(IType::I64, FType::F64))),
            0xbe => Ok(Instr::Numeric(NumericInstr::FReinterpretI(FType::F32, IType::I32))),
            0xbf => Ok(Instr::Numeric(NumericInstr::FReinterpretI(FType::F64, IType::I64))),
            0xc0 => Ok(Instr::Numeric(NumericInstr::IExtend8S(IType::I32))),
            0xc1 => Ok(Instr::Numeric(NumericInstr::IExtend16S(IType::I32))),
            0xc2 => Ok(Instr::Numeric(NumericInstr::IExtend8S(IType::I64))),
            0xc3 => Ok(Instr::Numeric(NumericInstr::IExtend16S(IType::I64))),
            0xc4 => Ok(Instr::Numeric(NumericInstr::I64Extend32)),

            0xFC => {
                let mut buf: [u8; 1] = [0];
                let n = reader.read(&mut buf)?;
                let byte = u8::from_le_bytes(buf);
                match n {
                    1 => match byte {
                        0x00 => Ok(Instr::Numeric(NumericInstr::ITruncSatS(IType::I32, FType::F32))),
                        0x01 => Ok(Instr::Numeric(NumericInstr::ITruncSatU(IType::I32, FType::F32))),
                        0x02 => Ok(Instr::Numeric(NumericInstr::ITruncSatS(IType::I32, FType::F64))),
                        0x03 => Ok(Instr::Numeric(NumericInstr::ITruncSatU(IType::I32, FType::F64))),
                        0x04 => Ok(Instr::Numeric(NumericInstr::ITruncSatS(IType::I64, FType::F32))),
                        0x05 => Ok(Instr::Numeric(NumericInstr::ITruncSatU(IType::I64, FType::F32))),
                        0x06 => Ok(Instr::Numeric(NumericInstr::ITruncSatS(IType::I64, FType::F64))),
                        0x07 => Ok(Instr::Numeric(NumericInstr::ITruncSatU(IType::I64, FType::F64))),
                        0x08 => Ok(Instr::Memory(MemoryInstr::Init(DataIdx::parse(reader)?))),
                        0x09 => Ok(Instr::Memory(MemoryInstr::DataDrop(DataIdx::parse(reader)?))),
                        0x0A => Ok(Instr::Memory(MemoryInstr::MemoryCopy)),
                        0x0B => Ok(Instr::Memory(MemoryInstr::MemoryFill)),

                        // TODO: two bytes!!!
                        _ => Err(InstrParseErr::InvalidInstr(byte))
                    }
                    n => Err(InstrParseErr::WrongNumBytesRead(Asked(1), Received(n)))
                }
            }
            0xFD => {
                let mut buf: [u8; 1] = [0];
                let n = reader.read(&mut buf)?;
                let byte = u8::from_le_bytes(buf);
                match n {
                    1 => match byte {
                        0x00 => Ok(Instr::Memory(MemoryInstr::V128Load(MemArg::parse(reader)?))),
                        0x01 => Ok(Instr::Memory(MemoryInstr::V128Load8x8S(MemArg::parse(reader)?))),
                        0x02 => Ok(Instr::Memory(MemoryInstr::V128Load8x8U(MemArg::parse(reader)?))),
                        0x03 => Ok(Instr::Memory(MemoryInstr::V128Load16x4S(MemArg::parse(reader)?))),
                        0x04 => Ok(Instr::Memory(MemoryInstr::V128Load16x4U(MemArg::parse(reader)?))),
                        0x05 => Ok(Instr::Memory(MemoryInstr::V128Load32x2S(MemArg::parse(reader)?))),
                        0x06 => Ok(Instr::Memory(MemoryInstr::V128Load32x2U(MemArg::parse(reader)?))),
                        0x07 => Ok(Instr::Memory(MemoryInstr::V128Load8Splat(MemArg::parse(reader)?))),
                        0x08 => Ok(Instr::Memory(MemoryInstr::V128Load16Splat(MemArg::parse(reader)?))),
                        0x09 => Ok(Instr::Memory(MemoryInstr::V128Load32Splat(MemArg::parse(reader)?))),
                        0x0A => Ok(Instr::Memory(MemoryInstr::V128Load64Splat(MemArg::parse(reader)?))),
                        0x0B => Ok(Instr::Memory(MemoryInstr::V128Store(MemArg::parse(reader)?))),

                        0x54 => Ok(Instr::Memory(MemoryInstr::V128Load8Lane(MemArg::parse(reader)?, LaneIdx::parse(reader)?))),
                        0x55 => Ok(Instr::Memory(MemoryInstr::V128Load16Lane(MemArg::parse(reader)?, LaneIdx::parse(reader)?))),
                        0x56 => Ok(Instr::Memory(MemoryInstr::V128Load32Lane(MemArg::parse(reader)?, LaneIdx::parse(reader)?))),
                        0x57 => Ok(Instr::Memory(MemoryInstr::V128Load64Lane(MemArg::parse(reader)?, LaneIdx::parse(reader)?))),
                        0x58 => Ok(Instr::Memory(MemoryInstr::V128Store8Lane(MemArg::parse(reader)?, LaneIdx::parse(reader)?))),
                        0x59 => Ok(Instr::Memory(MemoryInstr::V128Store16Lane(MemArg::parse(reader)?, LaneIdx::parse(reader)?))),
                        0x5A => Ok(Instr::Memory(MemoryInstr::V128Store32Lane(MemArg::parse(reader)?, LaneIdx::parse(reader)?))),
                        0x5B => Ok(Instr::Memory(MemoryInstr::V128Store64Lane(MemArg::parse(reader)?, LaneIdx::parse(reader)?))),
                        0x5C => Ok(Instr::Memory(MemoryInstr::V128Load32Zero(MemArg::parse(reader)?))),
                        0x5D => Ok(Instr::Memory(MemoryInstr::V128Load64Zero(MemArg::parse(reader)?))),

                        // TODO: two bytes!!!
                        _ => Err(InstrParseErr::InvalidInstr(byte))
                    }
                    n => Err(InstrParseErr::WrongNumBytesRead(Asked(1), Received(n)))
                }
            }
            _ => Err(InstrParseErr::InvalidInstr(byte))
        }
    }
}
