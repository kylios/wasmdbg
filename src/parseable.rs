use std::io::{BufReader, Read};

pub trait Parseable {
    fn parse(reader: &mut BufReader<dyn Read>) -> Self;
}