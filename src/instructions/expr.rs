use std::io::{BufReader, Read};

use crate::instructions::instr::Instr;
use crate::parseable::Parseable;

/*
 * Function bodies, initialization values for globals, elements
 * and offsets of element segments, and offsets of data segments
 * are given as expressions, which are sequences of instructions
 * terminated by an `end` marker.
 */
pub struct Expr(Vec<Instr>);

impl Parseable for Expr {
    fn parse(reader: &mut BufReader<dyn Read>) -> crate::parseable::Result<Self>
    where
        Self: Sized,
    {
        // TODO
        Ok(Expr(vec!()))
    }
}
