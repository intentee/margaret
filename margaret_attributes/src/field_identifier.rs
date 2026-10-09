use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use syn::Ident;

#[derive(Debug, Eq, PartialEq)]
pub enum FieldIdentifier {
    Named(Ident),
    Positional(usize),
}

impl Display for FieldIdentifier {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Named(identifier) => write!(formatter, "{identifier}"),
            Self::Positional(position) => write!(formatter, "{position}"),
        }
    }
}
