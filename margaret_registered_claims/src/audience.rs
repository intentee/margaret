use std::fmt::Display;
use std::fmt::Formatter;

use crate::audience_parsing::AudienceParsing;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Audience {
    value: String,
}

impl Audience {
    #[must_use]
    pub fn parse(value: &str) -> AudienceParsing {
        if value.is_empty() {
            AudienceParsing::Empty
        } else {
            AudienceParsing::Accepted(Self {
                value: value.to_string(),
            })
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl Display for Audience {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.value)
    }
}
