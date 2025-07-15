use std::result::Result;

#[cfg(test)]
mod tests;

mod parseable;
mod types;
mod module;
mod section;

use crate::module::Module;
use crate::parseable::ParseError;

use std::io::BufReader;
use std::fs::File;

fn main() -> Result<(), ParseError> {
    let file_path = "funcs.wasm";
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let module = match Module::parse(&mut reader) {
        Ok(module) => module,
        Err(err) => {
            return Err(err.into())
        }
    };
    
    println!("Version: {}", module.version);
    println!("Sections:");
    for section in module.sections() {
        println!("* {}", section.section_type());
        println!("{}", section);
    }
    Ok(())
}
