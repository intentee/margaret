use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::jwk_rejection::JwkRejection;
use crate::key_id::KeyId;

#[derive(Debug)]
pub enum KeySetRejection {
    DuplicateKeyId {
        kid: KeyId,
    },
    Key {
        index: usize,
        rejection: JwkRejection,
    },
}

impl Display for KeySetRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::DuplicateKeyId { kid } => write!(
                formatter,
                "more than one key in the set carries the key id '{kid}'"
            ),
            Self::Key { index, rejection } => {
                write!(
                    formatter,
                    "the key at index {index} is rejected: {rejection}"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::KeySetRejection;
    use crate::jwk_rejection::JwkRejection;
    use crate::key_id::KeyId;

    #[test]
    fn describes_every_rejection() {
        let described = [
            KeySetRejection::DuplicateKeyId {
                kid: KeyId::new("twice".to_string()),
            },
            KeySetRejection::Key {
                index: 2,
                rejection: JwkRejection::MissingKeyId,
            },
        ]
        .map(|rejection| rejection.to_string());

        assert!(described[0].contains("'twice'"));
        assert_eq!(
            described[1],
            "the key at index 2 is rejected: the key has no key id"
        );
    }
}
