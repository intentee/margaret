use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::key_disclosure::KeyDisclosure;

#[derive(Debug)]
pub struct DisclosedKey {
    pub disclosure: KeyDisclosure,
    pub index: usize,
}

impl Display for DisclosedKey {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let Self { disclosure, index } = self;

        write!(
            formatter,
            "the key at index {index} is distrusted: {disclosure}"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::DisclosedKey;
    use crate::key_disclosure::KeyDisclosure;

    #[test]
    fn describes_the_key_by_its_index_and_disclosure() {
        assert_eq!(
            DisclosedKey {
                disclosure: KeyDisclosure::SymmetricKey,
                index: 2,
            }
            .to_string(),
            "the key at index 2 is distrusted: the key publishes a symmetric secret"
        );
    }
}
