use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct KeyId {
    value: String,
}

impl KeyId {
    #[must_use]
    pub fn new(value: String) -> Self {
        Self { value }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl Display for KeyId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(&self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::KeyId;

    #[test]
    fn displays_the_key_id_as_written() {
        assert_eq!(KeyId::new("kid-1".to_string()).to_string(), "kid-1");
    }
}
