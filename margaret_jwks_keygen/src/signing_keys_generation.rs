use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::jwks_key_error::JwksKeyError;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SigningKeysGeneration {
    value: u64,
}

impl SigningKeysGeneration {
    pub const FIRST: Self = Self { value: 1 };

    #[must_use]
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    /// # Errors
    ///
    /// Returns `JwksKeyError::GenerationExhausted` when no generation follows this one.
    pub fn successor(self) -> std::result::Result<Self, JwksKeyError> {
        self.value
            .checked_add(1)
            .map(Self::new)
            .ok_or(JwksKeyError::GenerationExhausted { generation: self })
    }

    #[must_use]
    pub fn value(self) -> u64 {
        self.value
    }
}

impl Display for SigningKeysGeneration {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::SigningKeysGeneration;

    #[test]
    fn follows_a_generation_with_the_next_one() {
        assert_eq!(
            SigningKeysGeneration::FIRST
                .successor()
                .expect("the first generation has a successor"),
            SigningKeysGeneration::new(2)
        );
    }

    #[test]
    fn reports_the_last_generation_as_exhausted() {
        assert_eq!(
            SigningKeysGeneration::new(u64::MAX)
                .successor()
                .expect_err("the last generation has no successor")
                .to_string(),
            format!(
                "the signing keys reached their last generation {}",
                u64::MAX
            )
        );
    }
}
