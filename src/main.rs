
#[cfg(test)]
mod tests;

mod parseable;
mod types;
mod module;
mod section;

use crate::parseable::{Parseable, Result};
use crate::module::Module;

use std::io::BufReader;
use std::fs::File;

fn main() -> Result<()> {
    let file_path = "funcs.wasm";
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let module = Module::parse(&mut reader)?;
    
    println!("Version: {}", module.version);
    println!("Sections:");
    for section in module.sections {
        println!("* {}", section.section_type());
        println!("{}", section);
    }
    Ok(())
}
