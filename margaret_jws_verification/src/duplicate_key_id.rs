use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::key_id::KeyId;

#[derive(Debug)]
pub struct DuplicateKeyId {
    pub kid: KeyId,
}

impl Display for DuplicateKeyId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(
            formatter,
            "more than one verification key carries the key id '{}'",
            self.kid
        )
    }
}

#[cfg(test)]
mod tests {
    use super::DuplicateKeyId;
    use crate::key_id::KeyId;

    #[test]
    fn names_the_shared_key_id() {
        assert_eq!(
            DuplicateKeyId {
                kid: KeyId::new("twice".to_string()),
            }
            .to_string(),
            "more than one verification key carries the key id 'twice'"
        );
    }
}
