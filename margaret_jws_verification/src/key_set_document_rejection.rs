use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::duplicate_key_id::DuplicateKeyId;

#[derive(Debug)]
pub enum KeySetDocumentRejection {
    DuplicateKeyId(DuplicateKeyId),
    Malformed { source: serde_json::Error },
}

impl Display for KeySetDocumentRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::DuplicateKeyId(duplicate) => duplicate.fmt(formatter),
            Self::Malformed { source } => {
                write!(formatter, "the document is not a jwk set: {source}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::KeySetDocumentRejection;
    use crate::duplicate_key_id::DuplicateKeyId;
    use crate::key_id::KeyId;

    #[test]
    fn describes_every_rejection() {
        let described = [
            KeySetDocumentRejection::DuplicateKeyId(DuplicateKeyId {
                kid: KeyId::new("twice".to_string()),
            }),
            KeySetDocumentRejection::Malformed {
                source: serde_json::from_str::<u8>("x").expect_err("not json"),
            },
        ]
        .map(|rejection| rejection.to_string());

        assert_eq!(
            described[0],
            "more than one verification key carries the key id 'twice'"
        );
        assert!(described[1].starts_with("the document is not a jwk set: "));
    }
}
