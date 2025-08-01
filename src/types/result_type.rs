use std::io::{BufReader, Read};

use crate::parseable::{Parseable, Result};
use crate::types::val_type::ValType;

#[derive(Debug, Clone, PartialEq)]
pub struct ResultType(Vec<ValType>);

impl ResultType {
    pub fn vec(&self) -> &std::vec::Vec<ValType> {
        self.0.as_ref()
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, ValType> {
        self.0.iter()
    }
}

impl From<ResultType> for Vec<ValType> {
    fn from(value: ResultType) -> Self {
        value.0
    }
}

impl IntoIterator for ResultType {
    type Item = ValType;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a ResultType {
    type Item = &'a ValType;
    type IntoIter = std::slice::Iter<'a, ValType>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Parseable for ResultType {
    fn parse(reader: &mut BufReader<dyn Read>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(ResultType(Vec::<ValType>::parse(reader)?))
    }
}
