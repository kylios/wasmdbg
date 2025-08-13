use std::io::BufReader;

use crate::instructions::instr::{Instr, InstrParseErr};
use crate::instructions::instr::control::ControlInstr;
use crate::parseable::{ParseError, Parseable, ReadSeek};

/*
 * Function bodies, initialization values for globals, elements
 * and offsets of element segments, and offsets of data segments
 * are given as expressions, which are sequences of instructions
 * terminated by an `end` marker.
 */
pub struct Expr {
    instrs: Vec<Instr>
}

impl Parseable for Expr {
    fn parse(reader: &mut BufReader<dyn ReadSeek>) -> crate::parseable::Result<Self>
    where
        Self: Sized,
    {
        let mut instrs = Vec::<Instr>::new();
        loop {
            match Instr::parse(reader) {
                Ok(instr) => {
                    if matches!(instr, Instr::Control(ControlInstr::End)) {
                        break;
                    }
                    instrs.push(instr)
                },
                Err(err) => {
                    if matches!(err, InstrParseErr::InvalidInstr(_)) {
                        reader.seek_relative(-1)?;
                    } else {
                        return Err(ParseError::Other("An error occurred".to_string())); // TODO - make better
                    }
                }
            }
        }
        // TODO
        Ok(Expr {
            instrs
        })
    }
}
