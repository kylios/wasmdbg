use std::result::Result;

mod instructions;
mod module;
mod parseable;
mod section;
mod types;

use crate::module::Module;
use crate::parseable::ParseError;

use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), ParseError> {
    let file_path = "funcs.wasm";
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let module = match Module::parse(&mut reader) {
        Ok(module) => module,
        Err(err) => return Err(err.into()),
    };

    println!("Version: {}", module.version);
    println!("Sections:");
    for section in module.sections() {
        println!("* {}", section.section_type());
        println!("{}", section);
    }
    Ok(())
}
