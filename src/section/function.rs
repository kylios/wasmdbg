use std::fmt::Display;
use std::io::{BufReader, Read};
use std::result::Result;

use crate::parseable::{Asked, ParseError, Parseable, Received};
use crate::section::Section;
use crate::types::leb128::Leb128;
use crate::types::primitives::{Size, TypeIdx};
use crate::types::val_type::ValType;

/*
 * The `type` of a function declares its signature by reference to a type
 * defined in the module. The parameters of the function are referenced
 * through 0-based local indices in the function’s body; they are mutable.
 *
 * The `locals` declare a vector of mutable local variables and their types.
 * These variables are referenced through local indices in the function’s
 * body. The index of the first local is the smallest index not referencing
 * a parameter.
 *
 * The `expr` is an instruction sequence that upon termination must produce
 * a stack matching the function type’s result type.
 *
 * Functions are referenced through function indices, starting with the
 * smallest index not referencing a function import.
 *
 * TODO: there will be some validation needed to ensure this function correctly
 * references the indexes in the function's body and the imports.
 */
pub struct Func {
    r#type: TypeIdx,
    locals: Vec<ValType>,
    // TODO:
    //body: Expr,
}

impl Parseable for Func {
    fn parse(reader: &mut BufReader<dyn Read>) -> crate::parseable::Result<Self>
    where
        Self: Sized,
    {
        Ok(Func {
            r#type: TypeIdx::parse(reader)?,
            locals: Vec::<ValType>::parse(reader)?,
        })
    }
}

pub struct FunctionSec {
    size: Size,
}

impl Section for FunctionSec {
    fn section_type(&self) -> &str {
        "function"
    }

    fn size(&self) -> Size {
        self.size
    }
}

impl FunctionSec {
    pub fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let size = u32::from(Leb128::<u32>::parse(reader)?);

        // TODO: this is temporary code and should be replaced by
        // actual parsing. We are just consuming bytes for the sake
        // of testing!
        let bytes_remaining = usize::try_from(size).unwrap();
        let mut bytes_read = 0;
        let mut buf: [u8; 1] = [0; 1];
        loop {
            let n = reader.read(&mut buf)?;
            if n != 1 {
                return Err(ParseError::WrongNumBytesRead(Asked(1), Received(n)));
            }

            bytes_read += n;
            if bytes_read == bytes_remaining {
                break;
            }
        }

        Ok(FunctionSec { size: Size(size) })
    }
}

impl Display for FunctionSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "UNIMPLEMENTED")?;
        writeln!(f, "Size: {}", self.size)
    }
}
