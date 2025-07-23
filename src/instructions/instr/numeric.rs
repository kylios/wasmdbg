use std::fmt::Display;

use crate::types::num_type::{NumType, IType, FType};

/*
 * The class `iN` defines uninterpreted integers, whose signedness interpretation
 * can vary depending on context. In the abstract syntax, they are represented as
 * unsigned values. However, some operations convert them to signed based on a
 * two’s complement interpretation.
 */
// pub enum Integer {
//     U32(u32),
//     U64(u64),
//     S32(i32),
//     S64(i64),
//     I32(i32),
//     I64(i64)
// }

// pub enum FloatingPoint {
//     F32(f32),
//     F64(f64)
// }

// pub enum Number {
//     Integer(Integer),
//     FP(FloatingPoint)
// }

// TODO: if we want to more closely match the spec, we should group the instructions by:
//
// unop := iunop | funop | extendN_s
// binop := ibinop | fbinop
// testop := itestop
// relop := irelop | frelop
// cvtop := wrap | extend | trunc | trunc_sat | convert | demote | promote | reinterpret
//
// We would want to parameterize signedness (U | S), size (32 | 64), and type (I | F).
pub enum NumericInstr {
    Const(NumType),
    Clz(IType), Ctz(IType), Popcnt(IType),
    Add(NumType), Sub(NumType), Mul(NumType),
    DivU(IType), DivS(IType), Div(FType),
    RemU(IType), RemS(IType),
    And(IType), Or(IType), Xor(IType),
    Shl(IType), ShrU(IType), ShrS(IType),
    Rotl(IType), Rotr(IType),
    Min(FType), Max(FType), Copysign(FType),
    Abs(FType), Neg(FType), Sqrt(FType), Ceil(FType), Floor(FType), Trunc(FType), Nearest(FType),
    Eqz(IType), Eq(NumType), Ne(NumType),
    LtU(IType), LtS(IType), GtU(IType), GtS(IType), LeU(IType), LeS(IType), GeU(IType), GeS(IType),
    Lt(FType), Gt(FType), Le(FType), Ge(FType),
    IExtend8S(IType), IExtend16S(IType), I64Extend32, I32WrapI64, I64ExtendI32U, I64ExtendI32S,
    ITruncU(IType, FType), ITruncS(IType, FType),
    ITruncSatU(IType, FType), ITruncSatS(IType, FType),
    F32DemoteF64, F64PromoteF32, FConvertIU(FType, IType), FConvertIS(FType, IType),
    IReinterpretF(IType, FType), FReinterpretI(FType, IType)
}

impl Display for NumericInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            NumericInstr::Const(t) => match t {
                NumType::F(t) => match t {
                    FType::F32 => "f32.const",
                    FType::F64 => "f64.const"
                },
                NumType::I(t) => match t {
                    IType::I32 => "i32.const",
                    IType::I64 => "i64.const"
                }
            },
            NumericInstr::Clz(t) => match t {
                IType::I32 => "i32.clz",
                IType::I64 => "i64.clz"
            },
            NumericInstr::Ctz(t) => match t {
                IType::I32 => "i32.ctz",
                IType::I64 => "i64.ctz"
            },
            NumericInstr::Popcnt(t) => match t {
                IType::I32 => "i32.popcnt",
                IType::I64 => "i64.popcnt"
            },
            NumericInstr::Add(t) => match t {
                NumType::F(t) => match t {
                    FType::F32 => "f32.add",
                    FType::F64 => "f64.add"
                },
                NumType::I(t) => match t {
                    IType::I32 => "i32.add",
                    IType::I64 => "i64.add"
                }
            },
            NumericInstr::Sub(t) => match t {
                NumType::F(t) => match t {
                    FType::F32 => "f32.sub",
                    FType::F64 => "f64.sub"
                },
                NumType::I(t) => match t {
                    IType::I32 => "i32.sub",
                    IType::I64 => "i64.sub"
                }
            },
            NumericInstr::Mul(t) => match t {
                NumType::F(t) => match t {
                    FType::F32 => "f32.mul",
                    FType::F64 => "f64.mul"
                },
                NumType::I(t) => match t {
                    IType::I32 => "i32.mul",
                    IType::I64 => "i64.mul"
                }
            },
            NumericInstr::DivU(t) => match t {
                IType::I32 => "i32.div_u",
                IType::I64 => "i64.div_u"
            },
            NumericInstr::DivS(t) => match t {
                IType::I32 => "i32.div_s",
                IType::I64 => "i64.div_s"
            },
            NumericInstr::Div(t) => match t {
                FType::F32 => "f32.div",
                FType::F64 => "f64.div"
            },
            NumericInstr::RemU(t) => match t {
                IType::I32 => "i32.rem_u",
                IType::I64 => "i64.rem_u"
            },
            NumericInstr::RemS(t) => match t {
                IType::I32 => "i32.rem_s",
                IType::I64 => "i64.rem_s"
            },
            NumericInstr::And(t) => match t {
                IType::I32 => "i32.and",
                IType::I64 => "i64.and"
            },
            NumericInstr::Or(t) => match t {
                IType::I32 => "i32.or",
                IType::I64 => "i64.or"
            },
            NumericInstr::Xor(t) => match t {
                IType::I32 => "i32.xor",
                IType::I64 => "i64.xor"
            },
            NumericInstr::Shl(t) => match t {
                IType::I32 => "i32.shl",
                IType::I64 => "i64.shl"
            },
            NumericInstr::ShrU(t) => match t {
                IType::I32 => "i32.shr_u",
                IType::I64 => "i64.shr_u"
            },
            NumericInstr::ShrS(t) => match t {
                IType::I32 => "i32.shr_s",
                IType::I64 => "i64.shr_s"
            },
            NumericInstr::Rotl(t) => match t {
                IType::I32 => "i32.rotl",
                IType::I64 => "i64.rotl"
            },
            NumericInstr::Rotr(t) => match t {
                IType::I32 => "i32.rotr",
                IType::I64 => "i64.rotr"
            },
            NumericInstr::Min(t) => match t {
                FType::F32 => "f32.min",
                FType::F64 => "f64.min"
            },
            NumericInstr::Max(t) => match t {
                FType::F32 => "f32.max",
                FType::F64 => "f64.max"
            },
            NumericInstr::Copysign(t) => match t {
                FType::F32 => "f32.copysign",
                FType::F64 => "f64.copysign"
            },
            NumericInstr::Abs(t) => match t {
                FType::F32 => "f32.abs",
                FType::F64 => "f64.abs"
            },
            NumericInstr::Neg(t) => match t {
                FType::F32 => "f32.neg",
                FType::F64 => "f64.neg"
            },
            NumericInstr::Sqrt(t) => match t {
                FType::F32 => "f32.sqrt",
                FType::F64 => "f64.sqrt"
            },
            NumericInstr::Ceil(t) => match t {
                FType::F32 => "f32.ceil",
                FType::F64 => "f64.ceil"
            },
            NumericInstr::Floor(t) => match t {
                FType::F32 => "f32.floor",
                FType::F64 => "f64.floor"
            },
            NumericInstr::Trunc(t) => match t {
                FType::F32 => "f32.trunc",
                FType::F64 => "f64.trunc"
            },
            NumericInstr::Nearest(t) => match t {
                FType::F32 => "f32.nearest",
                FType::F64 => "f64.nearest"
            },
            NumericInstr::Eqz(t) => match t {
                IType::I32 => "i32.eqz",
                IType::I64 => "i64.eqz"
            },
            NumericInstr::Eq(t) => match t {
                NumType::F(t) => match t {
                    FType::F32 => "f32.eq",
                    FType::F64 => "f64.eq"
                },
                NumType::I(t) => match t {
                    IType::I32 => "i32.eq",
                    IType::I64 => "i64.eq"
                }
            },
            NumericInstr::Ne(t) => match t {
                NumType::F(t) => match t {
                    FType::F32 => "f32.ne",
                    FType::F64 => "f64.ne"
                },
                NumType::I(t) => match t {
                    IType::I32 => "i32.ne",
                    IType::I64 => "i64.ne"
                }
            },
            NumericInstr::LtU(t) => match t {
                IType::I32 => "i32.lt_u",
                IType::I64 => "i64.lt_u"
            },
            NumericInstr::LtS(t) => match t {
                IType::I32 => "i32.lt_s",
                IType::I64 => "i64.lt_s"
            },
            NumericInstr::GtU(t) => match t {
                IType::I32 => "i32.gt_u",
                IType::I64 => "i64.gt_u"
            },
            NumericInstr::GtS(t) => match t {
                IType::I32 => "i32.gt_s",
                IType::I64 => "i64.gt_s"
            },
            NumericInstr::LeU(t) => match t {
                IType::I32 => "i32.le_u",
                IType::I64 => "i64.le_u"
            },
            NumericInstr::LeS(t) => match t {
                IType::I32 => "i32.le_s",
                IType::I64 => "i64.le_s"
            },
            NumericInstr::GeU(t) => match t {
                IType::I32 => "i32.ge_u",
                IType::I64 => "i64.ge_u"
            },
            NumericInstr::GeS(t) => match t {
                IType::I32 => "i32.ge_s",
                IType::I64 => "i64.ge_s"
            },
            NumericInstr::Lt(t) => match t {
                FType::F32 => "f32.lt",
                FType::F64 => "f64.lt"
            },
            NumericInstr::Gt(t) => match t {
                FType::F32 => "f32.gt",
                FType::F64 => "f64.gt"
            },
            NumericInstr::Le(t) => match t {
                FType::F32 => "f32.le",
                FType::F64 => "f64.le"
            },
            NumericInstr::Ge(t) => match t {
                FType::F32 => "f32.ge",
                FType::F64 => "f64.ge"
            },
            NumericInstr::IExtend8S(t) => match t {
                IType::I32 => "i32.extend8_s",
                IType::I64 => "i64.extend8_s"
            },
            NumericInstr::IExtend16S(t) => match t {
                IType::I32 => "i32.extend16_s",
                IType::I64 => "i64.extend16_s"
            },
            NumericInstr::I64Extend32 => "i64.extend32_s",
            NumericInstr::I64ExtendI32S => "i64.extend_i32_s",
            NumericInstr::I64ExtendI32U => "i64.extend_i32_u",
            NumericInstr::I32WrapI64 => "i32.wrap_i34",
            NumericInstr::ITruncU(i, f) => match i {
                IType::I32 => match f {
                    FType::F32 => "i32.trunc_f32_u",
                    FType::F64 => "i32.trunc_f64_u"
                },
                IType::I64 => match f {
                    FType::F32 => "i64.trunc_f32_u",
                    FType::F64 => "i64.trunc_f64_u"
                }
            },
            NumericInstr::ITruncS(i, f) => match i {
                IType::I32 => match f {
                    FType::F32 => "i32.trunc_f32_s",
                    FType::F64 => "i32.trunc_f64_s"
                },
                IType::I64 => match f {
                    FType::F32 => "i64.trunc_f32_s",
                    FType::F64 => "i64.trunc_f64_s"
                }
            },
            NumericInstr::ITruncSatU(i, f) => match i {
                IType::I32 => match f {
                    FType::F32 => "i32.trunc_sat_f32_u",
                    FType::F64 => "i32.trunc_sat_f64_u"
                },
                IType::I64 => match f {
                    FType::F32 => "i64.trunc_sat_f32_u",
                    FType::F64 => "i64.trunc_sat_f64_u"
                }
            },
            NumericInstr::ITruncSatS(i, f) => match i {
                IType::I32 => match f {
                    FType::F32 => "i32.trunc_sat_f32_s",
                    FType::F64 => "i32.trunc_sat_f64_s"
                },
                IType::I64 => match f {
                    FType::F32 => "i64.trunc_sat_f32_s",
                    FType::F64 => "i64.trunc_sat_f64_s"
                }
            },
            NumericInstr::F32DemoteF64 => "f32.demote_f64",
            NumericInstr::F64PromoteF32 => "f64.promote_f32",
            NumericInstr::FConvertIU(f, i) => match f {
                FType::F32 => match i {
                    IType::I32 => "f32.convert_i32_u",
                    IType::I64 => "f32.convert_i64_u"
                },
                FType::F64 => match i {
                    IType::I32 => "f64.convert_i32_u",
                    IType::I64 => "f64.convert_i64_u"
                }
            },
            NumericInstr::FConvertIS(f, i) => match f {
                FType::F32 => match i {
                    IType::I32 => "f32.convert_i32_s",
                    IType::I64 => "f32.convert_i64_s"
                },
                FType::F64 => match i {
                    IType::I32 => "f64.convert_i32_s",
                    IType::I64 => "f64.convert_i64_s"
                }
            },
            NumericInstr::IReinterpretF(i, f) => match i {
                IType::I32 => match f {
                    FType::F32 => "i32.reinterpret_f32",
                    FType::F64 => "i32.reinterpret_f64"
                },
                IType::I64 => match f {
                    FType::F32 => "i64.reinterpret_f32",
                    FType::F64 => "i64.reinterpret_f64"
                }
            },
            NumericInstr::FReinterpretI(f, i) => match f {
                FType::F32 => match i {
                    IType::I32 => "f32.reinterpret_i32",
                    IType::I64 => "f32.reinterpret_i64"
                },
                FType::F64 => match i {
                    IType::I32 => "f64.reinterpret_i32",
                    IType::I64 => "f64.reinterpret_i64"
                }
            }
        };

        write!(f, "{}", s)
    }
}
