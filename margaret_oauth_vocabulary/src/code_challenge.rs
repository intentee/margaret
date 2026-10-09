use aws_lc_rs::digest::SHA256_OUTPUT_LEN;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use base64ct::Error as Base64Error;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error;

use margaret_token_digest::equal_in_constant_time::equal_in_constant_time;
use margaret_token_digest::token_digest::TokenDigest;

use crate::code_challenge_parsing::CodeChallengeParsing;
use crate::code_challenge_rejection::CodeChallengeRejection;
use crate::code_verifier::CodeVerifier;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeChallenge {
    digest: [u8; SHA256_OUTPUT_LEN],
}

impl CodeChallenge {
    #[must_use]
    pub fn of(verifier: &CodeVerifier) -> Self {
        Self {
            digest: *TokenDigest::of(verifier.secret()).as_bytes(),
        }
    }

    #[must_use]
    pub fn parse(value: &str) -> CodeChallengeParsing {
        let mut digest = [0; SHA256_OUTPUT_LEN];

        match Base64UrlUnpadded::decode(value, &mut digest) {
            Ok(decoded) if decoded.len() == SHA256_OUTPUT_LEN => {
                CodeChallengeParsing::Accepted(Self { digest })
            }
            Ok(_) | Err(Base64Error::InvalidLength) => {
                CodeChallengeParsing::Rejected(CodeChallengeRejection::WrongDigestLength)
            }
            Err(Base64Error::InvalidEncoding) => {
                CodeChallengeParsing::Rejected(CodeChallengeRejection::NotBase64Url)
            }
        }
    }

    #[must_use]
    pub fn admits(&self, verifier: &CodeVerifier) -> bool {
        equal_in_constant_time(&self.digest, Self::of(verifier).digest.as_slice())
    }

    #[must_use]
    pub fn wire(&self) -> String {
        Base64UrlUnpadded::encode_string(&self.digest)
    }
}

impl<'de> Deserialize<'de> for CodeChallenge {
    fn deserialize<TDeserializer: Deserializer<'de>>(
        deserializer: TDeserializer,
    ) -> Result<Self, TDeserializer::Error> {
        match Self::parse(&String::deserialize(deserializer)?) {
            CodeChallengeParsing::Accepted(challenge) => Ok(challenge),
            CodeChallengeParsing::Rejected(rejection) => Err(TDeserializer::Error::custom(
                format!("the code challenge is malformed: {rejection:?}"),
            )),
        }
    }
}

impl Serialize for CodeChallenge {
    fn serialize<TSerializer: Serializer>(
        &self,
        serializer: TSerializer,
    ) -> Result<TSerializer::Ok, TSerializer::Error> {
        serializer.serialize_str(&self.wire())
    }
}

#[cfg(test)]
mod tests {
    use super::CodeChallenge;
    use crate::code_challenge_parsing::CodeChallengeParsing;
    use crate::code_challenge_rejection::CodeChallengeRejection;
    use crate::code_verifier::CodeVerifier;

    const RFC_7636_VERIFIER: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
    const RFC_7636_CHALLENGE: &str = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";

    fn verifier(value: &str) -> CodeVerifier {
        serde_json::from_value(serde_json::json!(value)).expect("the fixture verifier is accepted")
    }

    fn challenge(value: &str) -> CodeChallenge {
        serde_json::from_value(serde_json::json!(value)).expect("the fixture challenge is accepted")
    }

    #[test]
    fn derives_the_rfc_7636_example_challenge() {
        assert_eq!(
            CodeChallenge::of(&verifier(RFC_7636_VERIFIER)).wire(),
            RFC_7636_CHALLENGE
        );
    }

    #[test]
    fn deserializes_a_wire_challenge_into_its_digest() {
        assert_eq!(
            challenge(RFC_7636_CHALLENGE),
            CodeChallenge::of(&verifier(RFC_7636_VERIFIER))
        );
    }

    #[test]
    fn admits_the_verifier_of_the_challenge() {
        assert!(challenge(RFC_7636_CHALLENGE).admits(&verifier(RFC_7636_VERIFIER)));
    }

    #[test]
    fn refuses_another_verifier() {
        assert!(!challenge(RFC_7636_CHALLENGE).admits(&verifier(&"a".repeat(43))));
    }

    #[test]
    fn rejects_a_challenge_that_is_not_base64url() {
        assert!(matches!(
            CodeChallenge::parse("E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw+cM"),
            CodeChallengeParsing::Rejected(rejection)
                if rejection == CodeChallengeRejection::NotBase64Url
        ));
    }

    #[test]
    fn rejects_a_challenge_of_a_shorter_digest() {
        assert!(matches!(
            CodeChallenge::parse("E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw"),
            CodeChallengeParsing::Rejected(rejection)
                if rejection == CodeChallengeRejection::WrongDigestLength
        ));
    }

    #[test]
    fn rejects_a_challenge_of_a_longer_digest() {
        assert!(matches!(
            CodeChallenge::parse(&format!("{RFC_7636_CHALLENGE}AAAA")),
            CodeChallengeParsing::Rejected(rejection)
                if rejection == CodeChallengeRejection::WrongDigestLength
        ));
    }

    #[test]
    fn refuses_to_deserialize_a_malformed_challenge() {
        assert!(
            serde_json::from_value::<CodeChallenge>(serde_json::json!("not-a-digest")).is_err()
        );
    }

    #[test]
    fn refuses_to_deserialize_a_challenge_that_is_not_a_string() {
        assert!(serde_json::from_value::<CodeChallenge>(serde_json::json!(43)).is_err());
    }

    #[test]
    fn serializes_the_wire_challenge() {
        assert_eq!(
            serde_json::to_value(challenge(RFC_7636_CHALLENGE)).expect("the challenge serializes"),
            serde_json::json!(RFC_7636_CHALLENGE)
        );
    }
}
