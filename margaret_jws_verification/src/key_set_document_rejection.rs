use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::key_id::KeyId;

#[derive(Debug)]
pub enum KeySetDocumentRejection {
    DuplicateKeyId { kid: KeyId },
    Malformed { source: serde_json::Error },
}

impl Display for KeySetDocumentRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::DuplicateKeyId { kid } => write!(
                formatter,
                "more than one usable key in the document carries the key id '{kid}'"
            ),
            Self::Malformed { source } => {
                write!(formatter, "the document is not a jwk set: {source}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::KeySetDocumentRejection;
    use crate::key_id::KeyId;

    #[test]
    fn describes_every_rejection() {
        let described = [
            KeySetDocumentRejection::DuplicateKeyId {
                kid: KeyId::new("twice".to_string()),
            },
            KeySetDocumentRejection::Malformed {
                source: serde_json::from_str::<u8>("x").expect_err("not json"),
            },
        ]
        .map(|rejection| rejection.to_string());

        assert_eq!(
            described[0],
            "more than one usable key in the document carries the key id 'twice'"
        );
        assert!(described[1].starts_with("the document is not a jwk set: "));
    }
}
