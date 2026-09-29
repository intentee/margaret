use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use serde::Deserialize;
use serde::Deserializer;
use serde::de::Error;

use crate::key_use::KeyUse;

const WIRE_NAMES: [&str; 8] = [
    "sign",
    "verify",
    "encrypt",
    "decrypt",
    "wrapKey",
    "unwrapKey",
    "deriveKey",
    "deriveBits",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyOperation {
    Sign,
    Verify,
    Encrypt,
    Decrypt,
    WrapKey,
    UnwrapKey,
    DeriveKey,
    DeriveBits,
}

impl KeyOperation {
    pub const ALL: [Self; 8] = [
        Self::Sign,
        Self::Verify,
        Self::Encrypt,
        Self::Decrypt,
        Self::WrapKey,
        Self::UnwrapKey,
        Self::DeriveKey,
        Self::DeriveBits,
    ];

    #[must_use]
    pub fn key_use(self) -> KeyUse {
        match self {
            Self::Sign | Self::Verify => KeyUse::Signature,
            Self::Encrypt
            | Self::Decrypt
            | Self::WrapKey
            | Self::UnwrapKey
            | Self::DeriveKey
            | Self::DeriveBits => KeyUse::Encryption,
        }
    }

    #[must_use]
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::Sign => WIRE_NAMES[0],
            Self::Verify => WIRE_NAMES[1],
            Self::Encrypt => WIRE_NAMES[2],
            Self::Decrypt => WIRE_NAMES[3],
            Self::WrapKey => WIRE_NAMES[4],
            Self::UnwrapKey => WIRE_NAMES[5],
            Self::DeriveKey => WIRE_NAMES[6],
            Self::DeriveBits => WIRE_NAMES[7],
        }
    }
}

impl Display for KeyOperation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(self.wire_name())
    }
}

impl<'wire> Deserialize<'wire> for KeyOperation {
    fn deserialize<Source: Deserializer<'wire>>(
        deserializer: Source,
    ) -> std::result::Result<Self, Source::Error> {
        let written = String::deserialize(deserializer)?;

        Self::ALL
            .into_iter()
            .find(|operation| operation.wire_name() == written)
            .ok_or_else(|| Source::Error::unknown_variant(&written, &WIRE_NAMES))
    }
}

#[cfg(test)]
mod tests {
    use super::KeyOperation;
    use crate::key_use::KeyUse;

    #[test]
    fn reads_every_registered_operation_by_its_wire_name() {
        for operation in KeyOperation::ALL {
            let written = format!("\"{}\"", operation.wire_name());

            assert_eq!(
                serde_json::from_str::<KeyOperation>(&written).expect("the wire name parses"),
                operation
            );
        }
    }

    #[test]
    fn rejects_an_operation_outside_the_registry() {
        assert!(serde_json::from_str::<KeyOperation>("\"attest\"").is_err());
    }

    #[test]
    fn rejects_an_operation_that_is_not_a_string() {
        assert!(serde_json::from_str::<KeyOperation>("1").is_err());
    }

    #[test]
    fn displays_the_wire_name() {
        assert_eq!(KeyOperation::WrapKey.to_string(), "wrapKey");
    }

    #[test]
    fn assigns_only_signing_and_verifying_to_the_signature_use() {
        let signature_operations = KeyOperation::ALL
            .into_iter()
            .filter(|operation| operation.key_use() == KeyUse::Signature)
            .collect::<Vec<_>>();

        assert_eq!(
            signature_operations,
            vec![KeyOperation::Sign, KeyOperation::Verify]
        );
    }
}
