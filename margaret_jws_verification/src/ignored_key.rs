use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::ignored_key_reason::IgnoredKeyReason;

#[derive(Debug)]
pub struct IgnoredKey {
    pub index: usize,
    pub reason: IgnoredKeyReason,
}

impl Display for IgnoredKey {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let Self { index, reason } = self;

        write!(formatter, "the key at index {index} is ignored: {reason}")
    }
}

#[cfg(test)]
mod tests {
    use super::IgnoredKey;
    use crate::ignored_key_reason::IgnoredKeyReason;

    #[test]
    fn describes_the_key_by_its_index_and_reason() {
        let ignored_key = IgnoredKey {
            index: 1,
            reason: IgnoredKeyReason::UnsupportedKeyType {
                kty: "OKP".to_string(),
            },
        };

        assert_eq!(
            ignored_key.to_string(),
            "the key at index 1 is ignored: the key is of the unsupported type 'OKP'"
        );
    }
}
