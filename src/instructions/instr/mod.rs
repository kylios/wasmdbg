pub mod control;
pub mod memory;
pub mod numeric;
pub mod parametric;
pub mod reference;
pub mod table;
pub mod variable;
pub mod vector;

use std::fmt::Display;
use std::io::{BufReader, Read};

use crate::instructions::instr::control::ControlInstr;
use crate::instructions::instr::memory::MemoryInstr;
use crate::instructions::instr::numeric::NumericInstr;
use crate::instructions::instr::parametric::ParametricInstr;
use crate::instructions::instr::reference::ReferenceInstr;
use crate::instructions::instr::table::TableInstr;
use crate::instructions::instr::variable::VariableInstr;
use crate::instructions::instr::vector::VectorInstr;
use crate::parseable::{ParseError, Parseable, ReadSeek};
use crate::types::leb128::Leb128;
use crate::types::mem_arg::MemArg;
use crate::types::num_type::{FType, IType, NumType};
use crate::types::primitives::{
    DataIdx, ElemIdx, FuncIdx, GlobalIdx, LabelIdx, LaneIdx, LocalIdx, TableIdx, TypeIdx,
};
use crate::types::ref_type::RefType;
use crate::types::val_type::ValType;

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
    ValType(ValType),
}

impl Parseable for BlockType {
    fn parse(reader: &mut BufReader<dyn ReadSeek>) -> Result<BlockType, ParseError>
    where
        Self: Sized,
    {
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
            BlockType::ValType(t) => write!(f, "{}", t),
        }
    }
}

#[cfg(test)]
mod block_type_tests {
    use super::*;
    use crate::types::num_type::NumType;
    use crate::types::ref_type::RefType;
    use crate::types::vec_type::VecType;
    use std::io::Cursor;

    #[test]
    fn test_parse_block_type_ok() {
        // Test parsing all `valtype` types, then test parsing
        // the value as a `typeidx`. When parsing `typeidx` values
        // that fall within the range `0x6F` - `0x7F` (the range of
        // `valtype` types), the `typeidx` should be represented as
        // two bytes: `0xEF 0x00` - `0xFF 0x00` (decimal values 111 - 127).
        let bytes: [u8; 11] = [
            0x6F, 0x70, 0x7B, 0x7C, 0x7D, 0x7E, 0x7F, 0xEF, 0x00, 0xFF, 0x00,
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
        assert_eq!(
            val,
            BlockType::ValType(ValType::Num(NumType::F(FType::F64)))
        );

        let result = BlockType::parse(&mut reader);
        assert!(result.is_ok());
        let val = result.expect("The parsed value");
        assert_eq!(
            val,
            BlockType::ValType(ValType::Num(NumType::F(FType::F32)))
        );

        let result = BlockType::parse(&mut reader);
        assert!(result.is_ok());
        let val = result.expect("The parsed value");
        assert_eq!(
            val,
            BlockType::ValType(ValType::Num(NumType::I(IType::I64)))
        );

        let result = BlockType::parse(&mut reader);
        assert!(result.is_ok());
        let val = result.expect("The parsed value");
        assert_eq!(
            val,
            BlockType::ValType(ValType::Num(NumType::I(IType::I32)))
        );

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
    ParseError(ParseError),
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
        Self: Sized,
    {
        let mut buf: [u8; 1] = [0];
        let n = reader.read(&mut buf)?;
        match n {
            1 => Instr::from(u8::from_le_bytes(buf), reader),
            n => Err(InstrParseErr::WrongNumBytesRead(Asked(1), Received(n))),
        }
    }

    fn from(byte: u8, reader: &mut BufReader<dyn ReadSeek>) -> Result<Instr, InstrParseErr> {
        match byte {
            0x00 => Ok(Instr::Control(ControlInstr::Unreachable)),
            0x01 => Ok(Instr::Control(ControlInstr::Nop)),
            0x02 => Ok(Instr::Control(ControlInstr::Block(BlockType::parse(
                reader,
            )?))),
            0x03 => Ok(Instr::Control(ControlInstr::Loop(BlockType::parse(
                reader,
            )?))),
            0x04 => Ok(Instr::Control(ControlInstr::If(BlockType::parse(reader)?))),
            0x05 => Ok(Instr::Control(ControlInstr::Else)),
            // 0x06 - 0x0A reserved
            0x0B => Ok(Instr::Control(ControlInstr::End)),
            0x0C => Ok(Instr::Control(ControlInstr::Br(LabelIdx::parse(reader)?))),
            0x0D => Ok(Instr::Control(ControlInstr::BrIf(LabelIdx::parse(reader)?))),
            0x0E => Ok(Instr::Control(ControlInstr::BrTable(
                Vec::<LabelIdx>::parse(reader)?,
                LabelIdx::parse(reader)?,
            ))),
            0x0F => Ok(Instr::Control(ControlInstr::Return)),
            0x10 => Ok(Instr::Control(ControlInstr::Call(FuncIdx::parse(reader)?))),
            0x11 => Ok(Instr::Control(ControlInstr::CallIndirect(
                TableIdx::parse(reader)?,
                TypeIdx::parse(reader)?,
            ))),
            // 0x12 - 0x19 reserved
            0x1A => Ok(Instr::Parametric(ParametricInstr::Drop)),
            0x1B => Ok(Instr::Parametric(ParametricInstr::Select(None))),
            0x1C => Ok(Instr::Parametric(ParametricInstr::Select(Some(
                ValType::parse(reader)?,
            )))),

            0x20 => Ok(Instr::Variable(VariableInstr::LocalGet(LocalIdx::parse(
                reader,
            )?))),
            0x21 => Ok(Instr::Variable(VariableInstr::LocalSet(LocalIdx::parse(
                reader,
            )?))),
            0x22 => Ok(Instr::Variable(VariableInstr::LocalTee(LocalIdx::parse(
                reader,
            )?))),
            0x23 => Ok(Instr::Variable(VariableInstr::GlobalGet(GlobalIdx::parse(
                reader,
            )?))),
            0x24 => Ok(Instr::Variable(VariableInstr::GlobalSet(GlobalIdx::parse(
                reader,
            )?))),
            0x25 => Ok(Instr::Table(TableInstr::Get(TableIdx::parse(reader)?))),
            0x26 => Ok(Instr::Table(TableInstr::Set(TableIdx::parse(reader)?))),
            // 0x27 reserved
            // Memory Instructions
            0x28 => Ok(Instr::Memory(MemoryInstr::I32Load(MemArg::parse(reader)?))),
            0x29 => Ok(Instr::Memory(MemoryInstr::I64Load(MemArg::parse(reader)?))),
            0x2A => Ok(Instr::Memory(MemoryInstr::F32Load(MemArg::parse(reader)?))),
            0x2B => Ok(Instr::Memory(MemoryInstr::F64Load(MemArg::parse(reader)?))),
            0x2C => Ok(Instr::Memory(MemoryInstr::I32Load8S(MemArg::parse(
                reader,
            )?))),
            0x2D => Ok(Instr::Memory(MemoryInstr::I32Load8U(MemArg::parse(
                reader,
            )?))),
            0x2E => Ok(Instr::Memory(MemoryInstr::I32Load16S(MemArg::parse(
                reader,
            )?))),
            0x2F => Ok(Instr::Memory(MemoryInstr::I32Load16U(MemArg::parse(
                reader,
            )?))),
            0x30 => Ok(Instr::Memory(MemoryInstr::I64Load8S(MemArg::parse(
                reader,
            )?))),
            0x31 => Ok(Instr::Memory(MemoryInstr::I64Load8U(MemArg::parse(
                reader,
            )?))),
            0x32 => Ok(Instr::Memory(MemoryInstr::I64Load16S(MemArg::parse(
                reader,
            )?))),
            0x33 => Ok(Instr::Memory(MemoryInstr::I64Load16U(MemArg::parse(
                reader,
            )?))),
            0x34 => Ok(Instr::Memory(MemoryInstr::I64Load32S(MemArg::parse(
                reader,
            )?))),
            0x35 => Ok(Instr::Memory(MemoryInstr::I64Load32U(MemArg::parse(
                reader,
            )?))),
            0x36 => Ok(Instr::Memory(MemoryInstr::I32Store(MemArg::parse(reader)?))),
            0x37 => Ok(Instr::Memory(MemoryInstr::I64Store(MemArg::parse(reader)?))),
            0x38 => Ok(Instr::Memory(MemoryInstr::F32Store(MemArg::parse(reader)?))),
            0x39 => Ok(Instr::Memory(MemoryInstr::F64Store(MemArg::parse(reader)?))),
            0x3A => Ok(Instr::Memory(MemoryInstr::I32Store8(MemArg::parse(
                reader,
            )?))),
            0x3B => Ok(Instr::Memory(MemoryInstr::I32Store16(MemArg::parse(
                reader,
            )?))),
            0x3C => Ok(Instr::Memory(MemoryInstr::I64Store8(MemArg::parse(
                reader,
            )?))),
            0x3D => Ok(Instr::Memory(MemoryInstr::I64Store16(MemArg::parse(
                reader,
            )?))),
            0x3E => Ok(Instr::Memory(MemoryInstr::I64Store32(MemArg::parse(
                reader,
            )?))),
            0x3F => Ok(Instr::Memory(MemoryInstr::MemorySize)),
            0x40 => Ok(Instr::Memory(MemoryInstr::MemoryGrow)),

            // Numeric Instructions
            0x41 => Ok(Instr::Numeric(NumericInstr::Const(Num::I32(
                Leb128::<i32>::parse(reader)?,
            )))),
            0x42 => Ok(Instr::Numeric(NumericInstr::Const(Num::I64(
                Leb128::<i64>::parse(reader)?,
            )))),
            // 0x43 => Ok(Instr::Numeric(NumericInstr::Const(NumType::F(FType::F32)))),
            // 0x44 => Ok(Instr::Numeric(NumericInstr::Const(NumType::F(FType::F64)))),
            0x45 => Ok(Instr::Numeric(NumericInstr::Eqz(IType::I32))),
            0x46 => Ok(Instr::Numeric(NumericInstr::Eq(NumType::I(IType::I32)))),
            0x47 => Ok(Instr::Numeric(NumericInstr::Ne(NumType::I(IType::I32)))),
            0x48 => Ok(Instr::Numeric(NumericInstr::LtS(IType::I32))),
            0x49 => Ok(Instr::Numeric(NumericInstr::LtU(IType::I32))),
            0x4A => Ok(Instr::Numeric(NumericInstr::GtS(IType::I32))),
            0x4B => Ok(Instr::Numeric(NumericInstr::GtU(IType::I32))),
            0x4C => Ok(Instr::Numeric(NumericInstr::LeS(IType::I32))),
            0x4D => Ok(Instr::Numeric(NumericInstr::LeU(IType::I32))),
            0x4E => Ok(Instr::Numeric(NumericInstr::GeS(IType::I32))),
            0x4F => Ok(Instr::Numeric(NumericInstr::GeU(IType::I32))),
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
            0x5A => Ok(Instr::Numeric(NumericInstr::GeU(IType::I64))),
            0x5B => Ok(Instr::Numeric(NumericInstr::Eq(NumType::F(FType::F32)))),
            0x5C => Ok(Instr::Numeric(NumericInstr::Ne(NumType::F(FType::F32)))),
            0x5D => Ok(Instr::Numeric(NumericInstr::Lt(FType::F32))),
            0x5E => Ok(Instr::Numeric(NumericInstr::Gt(FType::F32))),
            0x5F => Ok(Instr::Numeric(NumericInstr::Le(FType::F32))),
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
            0x6A => Ok(Instr::Numeric(NumericInstr::Add(NumType::I(IType::I32)))),
            0x6B => Ok(Instr::Numeric(NumericInstr::Sub(NumType::I(IType::I32)))),
            0x6C => Ok(Instr::Numeric(NumericInstr::Mul(NumType::I(IType::I32)))),
            0x6D => Ok(Instr::Numeric(NumericInstr::DivS(IType::I32))),
            0x6E => Ok(Instr::Numeric(NumericInstr::DivU(IType::I32))),
            0x6F => Ok(Instr::Numeric(NumericInstr::RemS(IType::I32))),
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
            0x7A => Ok(Instr::Numeric(NumericInstr::Ctz(IType::I64))),
            0x7B => Ok(Instr::Numeric(NumericInstr::Popcnt(IType::I64))),
            0x7C => Ok(Instr::Numeric(NumericInstr::Add(NumType::I(IType::I64)))),
            0x7D => Ok(Instr::Numeric(NumericInstr::Sub(NumType::I(IType::I64)))),
            0x7E => Ok(Instr::Numeric(NumericInstr::Mul(NumType::I(IType::I64)))),
            0x7F => Ok(Instr::Numeric(NumericInstr::DivS(IType::I64))),
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
            0x8A => Ok(Instr::Numeric(NumericInstr::Rotr(IType::I64))),
            0x8B => Ok(Instr::Numeric(NumericInstr::Abs(FType::F32))),
            0x8C => Ok(Instr::Numeric(NumericInstr::Neg(FType::F32))),
            0x8D => Ok(Instr::Numeric(NumericInstr::Ceil(FType::F32))),
            0x8E => Ok(Instr::Numeric(NumericInstr::Floor(FType::F32))),
            0x8F => Ok(Instr::Numeric(NumericInstr::Trunc(FType::F32))),
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
            0x9A => Ok(Instr::Numeric(NumericInstr::Neg(FType::F64))),
            0x9B => Ok(Instr::Numeric(NumericInstr::Ceil(FType::F64))),
            0x9C => Ok(Instr::Numeric(NumericInstr::Floor(FType::F64))),
            0x9D => Ok(Instr::Numeric(NumericInstr::Trunc(FType::F64))),
            0x9E => Ok(Instr::Numeric(NumericInstr::Nearest(FType::F64))),
            0x9F => Ok(Instr::Numeric(NumericInstr::Sqrt(FType::F64))),
            0xA0 => Ok(Instr::Numeric(NumericInstr::Add(NumType::F(FType::F64)))),
            0xA1 => Ok(Instr::Numeric(NumericInstr::Sub(NumType::F(FType::F64)))),
            0xA2 => Ok(Instr::Numeric(NumericInstr::Mul(NumType::F(FType::F64)))),
            0xA3 => Ok(Instr::Numeric(NumericInstr::Div(FType::F64))),
            0xA4 => Ok(Instr::Numeric(NumericInstr::Min(FType::F64))),
            0xA5 => Ok(Instr::Numeric(NumericInstr::Max(FType::F64))),
            0xA6 => Ok(Instr::Numeric(NumericInstr::Copysign(FType::F64))),
            0xA7 => Ok(Instr::Numeric(NumericInstr::I32WrapI64)),
            0xA8 => Ok(Instr::Numeric(NumericInstr::ITruncS(
                IType::I32,
                FType::F32,
            ))),
            0xA9 => Ok(Instr::Numeric(NumericInstr::ITruncU(
                IType::I32,
                FType::F32,
            ))),
            0xAA => Ok(Instr::Numeric(NumericInstr::ITruncS(
                IType::I32,
                FType::F64,
            ))),
            0xAB => Ok(Instr::Numeric(NumericInstr::ITruncU(
                IType::I32,
                FType::F64,
            ))),
            0xAC => Ok(Instr::Numeric(NumericInstr::I64ExtendI32S)),
            0xAD => Ok(Instr::Numeric(NumericInstr::I64ExtendI32U)),
            0xAE => Ok(Instr::Numeric(NumericInstr::ITruncS(
                IType::I64,
                FType::F32,
            ))),
            0xAF => Ok(Instr::Numeric(NumericInstr::ITruncU(
                IType::I64,
                FType::F32,
            ))),
            0xB0 => Ok(Instr::Numeric(NumericInstr::ITruncS(
                IType::I64,
                FType::F64,
            ))),
            0xB1 => Ok(Instr::Numeric(NumericInstr::ITruncU(
                IType::I64,
                FType::F64,
            ))),
            0xB2 => Ok(Instr::Numeric(NumericInstr::FConvertIS(
                FType::F32,
                IType::I32,
            ))),
            0xB3 => Ok(Instr::Numeric(NumericInstr::FConvertIU(
                FType::F32,
                IType::I32,
            ))),
            0xB4 => Ok(Instr::Numeric(NumericInstr::FConvertIS(
                FType::F32,
                IType::I64,
            ))),
            0xB5 => Ok(Instr::Numeric(NumericInstr::FConvertIU(
                FType::F32,
                IType::I64,
            ))),
            0xB6 => Ok(Instr::Numeric(NumericInstr::F32DemoteF64)),
            0xB7 => Ok(Instr::Numeric(NumericInstr::FConvertIS(
                FType::F64,
                IType::I32,
            ))),
            0xB8 => Ok(Instr::Numeric(NumericInstr::FConvertIU(
                FType::F64,
                IType::I32,
            ))),
            0xB9 => Ok(Instr::Numeric(NumericInstr::FConvertIS(
                FType::F64,
                IType::I64,
            ))),
            0xBA => Ok(Instr::Numeric(NumericInstr::FConvertIU(
                FType::F64,
                IType::I64,
            ))),
            0xBB => Ok(Instr::Numeric(NumericInstr::F64PromoteF32)),
            0xBC => Ok(Instr::Numeric(NumericInstr::IReinterpretF(
                IType::I32,
                FType::F32,
            ))),
            0xBD => Ok(Instr::Numeric(NumericInstr::IReinterpretF(
                IType::I64,
                FType::F64,
            ))),
            0xBE => Ok(Instr::Numeric(NumericInstr::FReinterpretI(
                FType::F32,
                IType::I32,
            ))),
            0xBF => Ok(Instr::Numeric(NumericInstr::FReinterpretI(
                FType::F64,
                IType::I64,
            ))),
            0xC0 => Ok(Instr::Numeric(NumericInstr::IExtend8S(IType::I32))),
            0xC1 => Ok(Instr::Numeric(NumericInstr::IExtend16S(IType::I32))),
            0xC2 => Ok(Instr::Numeric(NumericInstr::IExtend8S(IType::I64))),
            0xC3 => Ok(Instr::Numeric(NumericInstr::IExtend16S(IType::I64))),
            0xC4 => Ok(Instr::Numeric(NumericInstr::I64Extend32)),

            0xD0 => Ok(Instr::Reference(ReferenceInstr::Null(RefType::parse(
                reader,
            )?))),
            0xD1 => Ok(Instr::Reference(ReferenceInstr::IsNull)),
            0xD2 => Ok(Instr::Reference(ReferenceInstr::Func(FuncIdx::parse(
                reader,
            )?))),

            0xFC => {
                let mut buf: [u8; 1] = [0];
                let n = reader.read(&mut buf)?;
                let byte = u8::from_le_bytes(buf);
                match n {
                    1 => match byte {
                        0x00 => Ok(Instr::Numeric(NumericInstr::ITruncSatS(
                            IType::I32,
                            FType::F32,
                        ))),
                        0x01 => Ok(Instr::Numeric(NumericInstr::ITruncSatU(
                            IType::I32,
                            FType::F32,
                        ))),
                        0x02 => Ok(Instr::Numeric(NumericInstr::ITruncSatS(
                            IType::I32,
                            FType::F64,
                        ))),
                        0x03 => Ok(Instr::Numeric(NumericInstr::ITruncSatU(
                            IType::I32,
                            FType::F64,
                        ))),
                        0x04 => Ok(Instr::Numeric(NumericInstr::ITruncSatS(
                            IType::I64,
                            FType::F32,
                        ))),
                        0x05 => Ok(Instr::Numeric(NumericInstr::ITruncSatU(
                            IType::I64,
                            FType::F32,
                        ))),
                        0x06 => Ok(Instr::Numeric(NumericInstr::ITruncSatS(
                            IType::I64,
                            FType::F64,
                        ))),
                        0x07 => Ok(Instr::Numeric(NumericInstr::ITruncSatU(
                            IType::I64,
                            FType::F64,
                        ))),
                        0x08 => Ok(Instr::Memory(MemoryInstr::Init(DataIdx::parse(reader)?))),
                        0x09 => Ok(Instr::Memory(MemoryInstr::DataDrop(DataIdx::parse(
                            reader,
                        )?))),
                        0x0A => Ok(Instr::Memory(MemoryInstr::MemoryCopy)),
                        0x0B => Ok(Instr::Memory(MemoryInstr::MemoryFill)),

                        0x0C => Ok(Instr::Table(TableInstr::Init(
                            TableIdx::parse(reader)?,
                            ElemIdx::parse(reader)?,
                        ))),
                        0x0D => Ok(Instr::Table(TableInstr::Drop(ElemIdx::parse(reader)?))),
                        0x0E => Ok(Instr::Table(TableInstr::Copy(
                            TableIdx::parse(reader)?,
                            TableIdx::parse(reader)?,
                        ))),
                        0x0F => Ok(Instr::Table(TableInstr::Grow(TableIdx::parse(reader)?))),
                        0x10 => Ok(Instr::Table(TableInstr::Size(TableIdx::parse(reader)?))),
                        0x11 => Ok(Instr::Table(TableInstr::Fill(TableIdx::parse(reader)?))),

                        // TODO: two bytes!!!
                        _ => Err(InstrParseErr::InvalidInstr(byte)),
                    },
                    n => Err(InstrParseErr::WrongNumBytesRead(Asked(1), Received(n))),
                }
            }
            0xFD => {
                let mut buf: [u8; 1] = [0];
                let n = reader.read(&mut buf)?;
                let byte = u8::from_le_bytes(buf);
                match n {
                    1 => match byte {
                        0x00 => Ok(Instr::Memory(MemoryInstr::V128Load(MemArg::parse(reader)?))),
                        0x01 => Ok(Instr::Memory(MemoryInstr::V128Load8x8S(MemArg::parse(
                            reader,
                        )?))),
                        0x02 => Ok(Instr::Memory(MemoryInstr::V128Load8x8U(MemArg::parse(
                            reader,
                        )?))),
                        0x03 => Ok(Instr::Memory(MemoryInstr::V128Load16x4S(MemArg::parse(
                            reader,
                        )?))),
                        0x04 => Ok(Instr::Memory(MemoryInstr::V128Load16x4U(MemArg::parse(
                            reader,
                        )?))),
                        0x05 => Ok(Instr::Memory(MemoryInstr::V128Load32x2S(MemArg::parse(
                            reader,
                        )?))),
                        0x06 => Ok(Instr::Memory(MemoryInstr::V128Load32x2U(MemArg::parse(
                            reader,
                        )?))),
                        0x07 => Ok(Instr::Memory(MemoryInstr::V128Load8Splat(MemArg::parse(
                            reader,
                        )?))),
                        0x08 => Ok(Instr::Memory(MemoryInstr::V128Load16Splat(MemArg::parse(
                            reader,
                        )?))),
                        0x09 => Ok(Instr::Memory(MemoryInstr::V128Load32Splat(MemArg::parse(
                            reader,
                        )?))),
                        0x0A => Ok(Instr::Memory(MemoryInstr::V128Load64Splat(MemArg::parse(
                            reader,
                        )?))),
                        0x0B => Ok(Instr::Memory(MemoryInstr::V128Store(MemArg::parse(
                            reader,
                        )?))),

                        0x15 => Ok(Instr::Vector(VectorInstr::I8x16ExtractLaneS(
                            LaneIdx::parse(reader)?,
                        ))),
                        0x16 => Ok(Instr::Vector(VectorInstr::I8x16ExtractLaneU(
                            LaneIdx::parse(reader)?,
                        ))),
                        0x17 => Ok(Instr::Vector(VectorInstr::I8x16ReplaceLane(
                            LaneIdx::parse(reader)?,
                        ))),
                        0x18 => Ok(Instr::Vector(VectorInstr::I16x8ExtractLaneS(
                            LaneIdx::parse(reader)?,
                        ))),
                        0x19 => Ok(Instr::Vector(VectorInstr::I16x8ExtractLaneU(
                            LaneIdx::parse(reader)?,
                        ))),
                        0x1A => Ok(Instr::Vector(VectorInstr::I16x8ReplaceLane(
                            LaneIdx::parse(reader)?,
                        ))),
                        0x1B => Ok(Instr::Vector(VectorInstr::I32x4ExtractLane(
                            LaneIdx::parse(reader)?,
                        ))),
                        0x1C => Ok(Instr::Vector(VectorInstr::I32x4ReplaceLane(
                            LaneIdx::parse(reader)?,
                        ))),
                        0x1D => Ok(Instr::Vector(VectorInstr::I64x2ExtractLane(
                            LaneIdx::parse(reader)?,
                        ))),
                        0x1E => Ok(Instr::Vector(VectorInstr::I64x2ReplaceLane(
                            LaneIdx::parse(reader)?,
                        ))),
                        0x1F => Ok(Instr::Vector(VectorInstr::F32x4ExtractLane(
                            LaneIdx::parse(reader)?,
                        ))),
                        0x20 => Ok(Instr::Vector(VectorInstr::F32x4ReplaceLane(
                            LaneIdx::parse(reader)?,
                        ))),
                        0x21 => Ok(Instr::Vector(VectorInstr::F64x2ExtractLane(
                            LaneIdx::parse(reader)?,
                        ))),
                        0x22 => Ok(Instr::Vector(VectorInstr::F64x2ReplaceLane(
                            LaneIdx::parse(reader)?,
                        ))),
                        0x23 => Ok(Instr::Vector(VectorInstr::I8x16Eq)),
                        0x24 => Ok(Instr::Vector(VectorInstr::I8x16Ne)),
                        0x25 => Ok(Instr::Vector(VectorInstr::I8x16LtS)),
                        0x26 => Ok(Instr::Vector(VectorInstr::I8x16LtU)),
                        0x27 => Ok(Instr::Vector(VectorInstr::I8x16GtS)),
                        0x28 => Ok(Instr::Vector(VectorInstr::I8x16GtU)),
                        0x29 => Ok(Instr::Vector(VectorInstr::I8x16LeS)),
                        0x2a => Ok(Instr::Vector(VectorInstr::I8x16LeU)),
                        0x2b => Ok(Instr::Vector(VectorInstr::I8x16GeS)),
                        0x2c => Ok(Instr::Vector(VectorInstr::I8x16GeU)),
                        0x2d => Ok(Instr::Vector(VectorInstr::I16x8Eq)),
                        0x2e => Ok(Instr::Vector(VectorInstr::I16x8Ne)),
                        0x2f => Ok(Instr::Vector(VectorInstr::I16x8LtS)),
                        0x30 => Ok(Instr::Vector(VectorInstr::I16x8LtU)),
                        0x31 => Ok(Instr::Vector(VectorInstr::I16x8GtS)),
                        0x32 => Ok(Instr::Vector(VectorInstr::I16x8GtU)),
                        0x33 => Ok(Instr::Vector(VectorInstr::I16x8LeS)),
                        0x34 => Ok(Instr::Vector(VectorInstr::I16x8LeU)),
                        0x35 => Ok(Instr::Vector(VectorInstr::I16x8GeS)),
                        0x36 => Ok(Instr::Vector(VectorInstr::I16x8GeU)),
                        0x37 => Ok(Instr::Vector(VectorInstr::I32x4Eq)),
                        0x38 => Ok(Instr::Vector(VectorInstr::I32x4Ne)),
                        0x39 => Ok(Instr::Vector(VectorInstr::I32x4LtS)),
                        0x3a => Ok(Instr::Vector(VectorInstr::I32x4LtU)),
                        0x3b => Ok(Instr::Vector(VectorInstr::I32x4GtS)),
                        0x3c => Ok(Instr::Vector(VectorInstr::I32x4GtU)),
                        0x3d => Ok(Instr::Vector(VectorInstr::I32x4LeS)),
                        0x3e => Ok(Instr::Vector(VectorInstr::I32x4LeU)),
                        0x3f => Ok(Instr::Vector(VectorInstr::I32x4GeS)),
                        0x40 => Ok(Instr::Vector(VectorInstr::I32x4GeU)),
                        0x41 => Ok(Instr::Vector(VectorInstr::F32x4Eq)),
                        0x42 => Ok(Instr::Vector(VectorInstr::F32x4Ne)),
                        0x43 => Ok(Instr::Vector(VectorInstr::F32x4Lt)),
                        0x44 => Ok(Instr::Vector(VectorInstr::F32x4Gt)),
                        0x45 => Ok(Instr::Vector(VectorInstr::F32x4Le)),
                        0x46 => Ok(Instr::Vector(VectorInstr::F32x4Ge)),
                        0x47 => Ok(Instr::Vector(VectorInstr::F64x2Eq)),
                        0x48 => Ok(Instr::Vector(VectorInstr::F64x2Ne)),
                        0x49 => Ok(Instr::Vector(VectorInstr::F64x2Lt)),
                        0x4a => Ok(Instr::Vector(VectorInstr::F64x2Gt)),
                        0x4b => Ok(Instr::Vector(VectorInstr::F64x2Le)),
                        0x4c => Ok(Instr::Vector(VectorInstr::F64x2Ge)),
                        0x4d => Ok(Instr::Vector(VectorInstr::V128Not)),
                        0x4e => Ok(Instr::Vector(VectorInstr::V128And)),
                        0x4f => Ok(Instr::Vector(VectorInstr::V128AndNot)),
                        0x50 => Ok(Instr::Vector(VectorInstr::V128Or)),
                        0x51 => Ok(Instr::Vector(VectorInstr::V128Xor)),
                        0x52 => Ok(Instr::Vector(VectorInstr::V128BitSelect)),
                        0x53 => Ok(Instr::Vector(VectorInstr::V128AnyTrue)),

                        // Memory Instructions
                        0x54 => Ok(Instr::Memory(MemoryInstr::V128Load8Lane(
                            MemArg::parse(reader)?,
                            LaneIdx::parse(reader)?,
                        ))),
                        0x55 => Ok(Instr::Memory(MemoryInstr::V128Load16Lane(
                            MemArg::parse(reader)?,
                            LaneIdx::parse(reader)?,
                        ))),
                        0x56 => Ok(Instr::Memory(MemoryInstr::V128Load32Lane(
                            MemArg::parse(reader)?,
                            LaneIdx::parse(reader)?,
                        ))),
                        0x57 => Ok(Instr::Memory(MemoryInstr::V128Load64Lane(
                            MemArg::parse(reader)?,
                            LaneIdx::parse(reader)?,
                        ))),
                        0x58 => Ok(Instr::Memory(MemoryInstr::V128Store8Lane(
                            MemArg::parse(reader)?,
                            LaneIdx::parse(reader)?,
                        ))),
                        0x59 => Ok(Instr::Memory(MemoryInstr::V128Store16Lane(
                            MemArg::parse(reader)?,
                            LaneIdx::parse(reader)?,
                        ))),
                        0x5A => Ok(Instr::Memory(MemoryInstr::V128Store32Lane(
                            MemArg::parse(reader)?,
                            LaneIdx::parse(reader)?,
                        ))),
                        0x5B => Ok(Instr::Memory(MemoryInstr::V128Store64Lane(
                            MemArg::parse(reader)?,
                            LaneIdx::parse(reader)?,
                        ))),
                        0x5C => Ok(Instr::Memory(MemoryInstr::V128Load32Zero(MemArg::parse(
                            reader,
                        )?))),
                        0x5D => Ok(Instr::Memory(MemoryInstr::V128Load64Zero(MemArg::parse(
                            reader,
                        )?))),

                        0x5E => Ok(Instr::Vector(VectorInstr::F32x4DemoteF64x2Zero)),
                        0x5F => Ok(Instr::Vector(VectorInstr::F64x2PromoteLowF32x4)),
                        0x60 => Ok(Instr::Vector(VectorInstr::I8x16Abs)),
                        0x61 => Ok(Instr::Vector(VectorInstr::I8x16Neg)),
                        0x62 => Ok(Instr::Vector(VectorInstr::I8x16Popcnt)),
                        0x63 => Ok(Instr::Vector(VectorInstr::AllTrue(vector::IShape::I8x16))),
                        0x64 => Ok(Instr::Vector(VectorInstr::Bitmask(vector::IShape::I8x16))),
                        0x65 => Ok(Instr::Vector(VectorInstr::I8x16NarrowI16x8S)),
                        0x66 => Ok(Instr::Vector(VectorInstr::I8x16NarrowI16x8U)),
                        0x67 => Ok(Instr::Vector(VectorInstr::Ceil(vector::FShape::F32x4))),
                        0x68 => Ok(Instr::Vector(VectorInstr::Floor(vector::FShape::F32x4))),
                        0x69 => Ok(Instr::Vector(VectorInstr::Trunc(vector::FShape::F32x4))),
                        0x6A => Ok(Instr::Vector(VectorInstr::Nearest(vector::FShape::F32x4))),
                        0x6B => Ok(Instr::Vector(VectorInstr::I8x16Shl)),
                        0x6C => Ok(Instr::Vector(VectorInstr::I8x16ShrS)),
                        0x6D => Ok(Instr::Vector(VectorInstr::I8x16ShrU)),
                        0x6E => Ok(Instr::Vector(VectorInstr::I8x16Add)),
                        0x6F => Ok(Instr::Vector(VectorInstr::I8x16AddSatS)),
                        0x70 => Ok(Instr::Vector(VectorInstr::I8x16AddSatU)),
                        0x71 => Ok(Instr::Vector(VectorInstr::I8x16Sub)),
                        0x72 => Ok(Instr::Vector(VectorInstr::I8x16SubSatS)),
                        0x73 => Ok(Instr::Vector(VectorInstr::I8x16SubSatU)),
                        0x74 => Ok(Instr::Vector(VectorInstr::Ceil(vector::FShape::F64x2))),
                        0x75 => Ok(Instr::Vector(VectorInstr::Floor(vector::FShape::F64x2))),
                        0x76 => Ok(Instr::Vector(VectorInstr::I8x16MinS)),
                        0x77 => Ok(Instr::Vector(VectorInstr::I8x16MinU)),
                        0x78 => Ok(Instr::Vector(VectorInstr::I8x16MaxS)),
                        0x79 => Ok(Instr::Vector(VectorInstr::I8x16MaxU)),
                        0x7A => Ok(Instr::Vector(VectorInstr::Trunc(vector::FShape::F64x2))),
                        0x7B => Ok(Instr::Vector(VectorInstr::I8x16AvgrU)),
                        0x7C => Ok(Instr::Vector(VectorInstr::I16x8ExtaddPairwiseI8x16S)),
                        0x7D => Ok(Instr::Vector(VectorInstr::I16x8ExtaddPairwiseI8x16U)),
                        0x7E => Ok(Instr::Vector(VectorInstr::I32x4ExtaddPairwiseI16x8S)),
                        0x7F => Ok(Instr::Vector(VectorInstr::I32x4ExtaddPairwiseI16x8U)),

                        0x80 => {
                            let mut buf: [u8; 1] = [0];
                            let n = reader.read(&mut buf)?;
                            let byte = u8::from_le_bytes(buf);
                            match n {
                                1 => match byte {
                                    // TODO
                                },
                                n => Err(InstrParseErr::WrongNumBytesRead(Asked(1), Received(n))),
                            }
                        }

                        // TODO: two bytes!!!
                        _ => Err(InstrParseErr::InvalidInstr(byte)),
                    },
                    n => Err(InstrParseErr::WrongNumBytesRead(Asked(1), Received(n))),
                }
            }
            _ => Err(InstrParseErr::InvalidInstr(byte)),
        }
    }
}
