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

use crate::parseable::{Asked, ParseError, Parseable, Received};
use crate::types::leb128::Leb128;
use crate::types::num_type::{NumType, IType, FType};
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

// TODO: can we implement Parseable for this type? The match statement above is non-exhaustive,
// so we will need to return some type that indicates a byte is not a NumericInstr. Perhaps
// the parsing should occur at a higher level where the matching can be more exhaustive, and
// the correct instruction returned, whether it's a NumericInstr or something else.
impl Parseable for Instr {
    fn parse(reader: &mut BufReader<dyn Read>) -> crate::parseable::Result<Self>
        where
            Self: Sized {
        let mut buf: [u8; 1] = [0];
        let n = reader.read(&mut buf)?;
        match n {
            1 => match Instr::from(u8::from_le_bytes(buf), reader) {
                Ok(instr) => Ok(instr),
                _ => Err(ParseError::Other("Invalid instruction".to_string()))  // TODO: should return a custom error type for this
            },
            n => Err(ParseError::WrongNumBytesRead(Asked(1), Received(n)))
        }
    }
}

impl Instr {
    fn from(byte: u8, reader: &mut BufReader<dyn Read>) -> Result<Instr, ParseError> {
        match byte {
            0x00 => Ok(Instr::Control(ControlInstr::Unreachable)),
            0x01 => Ok(Instr::Control(ControlInstr::Nop)),

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
            _ => Err(ParseError::Other(format!("Invalid Instruction: {}", byte)))
        }
    }
}
