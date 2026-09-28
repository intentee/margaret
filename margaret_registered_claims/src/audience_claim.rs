use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use serde::Deserialize;
use serde_json::Value;

use crate::audience::Audience;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum AudienceClaim {
    Multiple(Vec<String>),
    Single(String),
}

impl AudienceClaim {
    #[must_use]
    pub fn is_exactly(&self, audience: &Audience) -> bool {
        match self {
            Self::Multiple(values) => {
                matches!(values.as_slice(), [value] if value == audience.as_str())
            }
            Self::Single(value) => value == audience.as_str(),
        }
    }

    #[must_use]
    pub fn to_json(&self) -> Value {
        match self {
            Self::Multiple(values) => Value::Array(
                values
                    .iter()
                    .map(|value| Value::String(value.clone()))
                    .collect(),
            ),
            Self::Single(value) => Value::String(value.clone()),
        }
    }
}

impl Display for AudienceClaim {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Multiple(values) => write!(formatter, "[{}]", values.join(", ")),
            Self::Single(value) => formatter.write_str(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AudienceClaim;

    #[test]
    fn describes_a_single_audience_as_written() {
        assert_eq!(
            AudienceClaim::Single("ours".to_string()).to_string(),
            "ours"
        );
    }
}
