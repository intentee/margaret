use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug, Eq, PartialEq)]
pub enum FieldIdentifier {
    Named(String),
    Positional(usize),
}

impl Display for FieldIdentifier {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Named(name) => formatter.write_str(name),
            Self::Positional(position) => write!(formatter, "{position}"),
        }
    }
}
