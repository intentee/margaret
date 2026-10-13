use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClientIdRejection {
    Empty,
    InvisibleCharacter,
}

impl Display for ClientIdRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(match self {
            Self::Empty => "the client identifier is empty",
            Self::InvisibleCharacter => {
                "the client identifier contains a character outside the visible ascii range"
            }
        })
    }
}
