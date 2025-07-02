
#[cfg(test)]
mod tests;

mod parseable;
mod types;
mod module;

use crate::parseable::Parseable;
use crate::module::Module;

use std::io::{self, BufReader, prelude::*};
use std::fs::File;

fn main() -> io::Result<()> {
    let file_path = "funcs.wasm";
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let module = Module::parse(&mut reader);
    
    println!("Version: {}", module.version);
    Ok(())
}
