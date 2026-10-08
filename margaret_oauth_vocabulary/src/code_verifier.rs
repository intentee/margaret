use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::RangeInclusive;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error;
use zeroize::Zeroizing;

use margaret_token_digest::random_token::random_token;

use crate::code_verifier_parsing::CodeVerifierParsing;
use crate::code_verifier_rejection::CodeVerifierRejection;

const VERIFIER_LENGTH: RangeInclusive<usize> = 43..=128;

fn is_unreserved(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~')
}

pub struct CodeVerifier {
    secret: Zeroizing<String>,
}

impl CodeVerifier {
    #[must_use]
    pub fn generate() -> Self {
        Self {
            secret: Zeroizing::new(random_token()),
        }
    }

    #[must_use]
    pub fn parse(value: &str) -> CodeVerifierParsing {
        let length = value.len();

        if length < *VERIFIER_LENGTH.start() {
            return CodeVerifierParsing::Rejected(CodeVerifierRejection::TooShort { length });
        }

        if length > *VERIFIER_LENGTH.end() {
            return CodeVerifierParsing::Rejected(CodeVerifierRejection::TooLong { length });
        }

        match value.bytes().position(|byte| !is_unreserved(byte)) {
            Some(position) => {
                CodeVerifierParsing::Rejected(CodeVerifierRejection::ReservedCharacter { position })
            }
            None => CodeVerifierParsing::Accepted(Self {
                secret: Zeroizing::new(value.to_string()),
            }),
        }
    }

    #[must_use]
    pub fn secret(&self) -> &str {
        &self.secret
    }
}

impl Debug for CodeVerifier {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("CodeVerifier")
            .finish_non_exhaustive()
    }
}

impl<'de> Deserialize<'de> for CodeVerifier {
    fn deserialize<TDeserializer: Deserializer<'de>>(
        deserializer: TDeserializer,
    ) -> std::result::Result<Self, TDeserializer::Error> {
        let value = Zeroizing::new(String::deserialize(deserializer)?);

        match Self::parse(&value) {
            CodeVerifierParsing::Accepted(verifier) => Ok(verifier),
            CodeVerifierParsing::Rejected(rejection) => Err(TDeserializer::Error::custom(format!(
                "the code verifier is malformed: {rejection:?}"
            ))),
        }
    }
}

impl Serialize for CodeVerifier {
    fn serialize<TSerializer: Serializer>(
        &self,
        serializer: TSerializer,
    ) -> std::result::Result<TSerializer::Ok, TSerializer::Error> {
        serializer.serialize_str(&self.secret)
    }
}

#[cfg(test)]
mod tests {
    use super::CodeVerifier;
    use crate::code_verifier_parsing::CodeVerifierParsing;
    use crate::code_verifier_rejection::CodeVerifierRejection;

    const RFC_7636_VERIFIER: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";

    #[test]
    fn accepts_the_rfc_7636_example_verifier() {
        assert!(matches!(
            CodeVerifier::parse(RFC_7636_VERIFIER),
            CodeVerifierParsing::Accepted(verifier) if verifier.secret() == RFC_7636_VERIFIER
        ));
    }

    #[test]
    fn accepts_every_unreserved_character() {
        let value = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";

        assert!(matches!(
            CodeVerifier::parse(value),
            CodeVerifierParsing::Accepted(verifier) if verifier.secret() == value
        ));
    }

    #[test]
    fn rejects_a_verifier_shorter_than_43_characters() {
        assert!(matches!(
            CodeVerifier::parse(&"a".repeat(42)),
            CodeVerifierParsing::Rejected(rejection)
                if rejection == CodeVerifierRejection::TooShort { length: 42 }
        ));
    }

    #[test]
    fn rejects_a_verifier_longer_than_128_characters() {
        assert!(matches!(
            CodeVerifier::parse(&"a".repeat(129)),
            CodeVerifierParsing::Rejected(rejection)
                if rejection == CodeVerifierRejection::TooLong { length: 129 }
        ));
    }

    #[test]
    fn accepts_verifiers_at_both_length_bounds() {
        assert!(matches!(
            CodeVerifier::parse(&"a".repeat(43)),
            CodeVerifierParsing::Accepted(verifier) if verifier.secret().len() == 43
        ));
        assert!(matches!(
            CodeVerifier::parse(&"a".repeat(128)),
            CodeVerifierParsing::Accepted(verifier) if verifier.secret().len() == 128
        ));
    }

    #[test]
    fn rejects_a_reserved_character_at_its_position() {
        assert!(matches!(
            CodeVerifier::parse(&format!("{RFC_7636_VERIFIER}/")),
            CodeVerifierParsing::Rejected(rejection)
                if rejection == CodeVerifierRejection::ReservedCharacter { position: 43 }
        ));
    }

    #[test]
    fn rejects_a_multibyte_character() {
        assert!(matches!(
            CodeVerifier::parse(&format!("{}é", "a".repeat(42))),
            CodeVerifierParsing::Rejected(rejection)
                if rejection == CodeVerifierRejection::ReservedCharacter { position: 42 }
        ));
    }

    #[test]
    fn generates_an_acceptable_verifier() {
        let generated = CodeVerifier::generate();

        assert!(matches!(
            CodeVerifier::parse(generated.secret()),
            CodeVerifierParsing::Accepted(parsed) if parsed.secret() == generated.secret()
        ));
    }

    #[test]
    fn deserializes_an_acceptable_verifier() {
        let verifier: CodeVerifier = serde_json::from_value(serde_json::json!(RFC_7636_VERIFIER))
            .expect("the verifier deserializes");

        assert_eq!(verifier.secret(), RFC_7636_VERIFIER);
    }

    #[test]
    fn refuses_to_deserialize_a_malformed_verifier() {
        assert!(serde_json::from_value::<CodeVerifier>(serde_json::json!("short")).is_err());
    }

    #[test]
    fn refuses_to_deserialize_a_verifier_that_is_not_a_string() {
        assert!(serde_json::from_value::<CodeVerifier>(serde_json::json!(43)).is_err());
    }

    #[test]
    fn serializes_the_secret() {
        assert_eq!(
            serde_json::to_value(CodeVerifier::generate())
                .expect("the verifier serializes")
                .as_str()
                .map(str::len),
            Some(43)
        );
    }

    #[test]
    fn hides_the_secret_from_debug_output() {
        assert_eq!(
            format!("{:?}", CodeVerifier::generate()),
            "CodeVerifier { .. }"
        );
    }
}
