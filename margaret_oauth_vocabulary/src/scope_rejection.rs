use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScopeRejection {
    Character,
    Empty,
}

impl Display for ScopeRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(match self {
            Self::Character => {
                "the scope contains a character outside the scope token grammar of RFC 6749"
            }
            Self::Empty => "the scope is empty",
        })
    }
}
