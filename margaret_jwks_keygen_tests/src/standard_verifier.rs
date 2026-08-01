use anyhow::Context as _;
use anyhow::Result;
use anyhow::anyhow;
use jsonwebtoken::Algorithm;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use jsonwebtoken::decode_header;
use jsonwebtoken::jwk::Jwk;
use jsonwebtoken::jwk::JwkSet;
use serde::de::DeserializeOwned;

use margaret_jwt_claims::expected_claims::ExpectedClaims;

use crate::signing_algorithm::signing_algorithm;

fn declared_algorithm(jwk: &Jwk) -> Result<Algorithm> {
    signing_algorithm(
        jwk.common
            .key_algorithm
            .ok_or_else(|| anyhow!("the published key declares no alg"))?,
    )
}

fn declared_key_id(jwk: &Jwk) -> Result<&String> {
    jwk.common
        .key_id
        .as_ref()
        .ok_or_else(|| anyhow!("the published key declares no kid"))
}

pub struct StandardVerifier {
    published: JwkSet,
}

impl StandardVerifier {
    /// # Errors
    ///
    /// Returns an error when the document is not a jwk set.
    pub fn from_document(document: &str) -> Result<Self> {
        Ok(Self {
            published: serde_json::from_str(document)
                .context("the published document is not a jwk set")?,
        })
    }

    /// # Errors
    ///
    /// Returns an error when the token cannot be verified against the published set.
    pub fn decode<TClaims: DeserializeOwned>(
        &self,
        token: &str,
        ExpectedClaims { audience, issuer }: &ExpectedClaims,
    ) -> Result<TClaims> {
        let kid = decode_header(token)?
            .kid
            .ok_or_else(|| anyhow!("the token header carries no kid"))?;
        let jwk = self
            .published
            .find(&kid)
            .ok_or_else(|| anyhow!("the signing key '{kid}' is not published"))?;
        let mut validation = Validation::new(declared_algorithm(jwk)?);

        validation.set_audience(&[audience]);
        validation.set_issuer(&[issuer]);
        validation.validate_exp = true;

        Ok(decode::<TClaims>(token, &DecodingKey::from_jwk(jwk)?, &validation)?.claims)
    }

    /// # Errors
    ///
    /// Returns an error when any published key lacks a kid or an alg, or cannot
    /// be turned into a decoding key.
    pub fn load_every_key(&self) -> Result<Vec<String>> {
        let mut loaded = Vec::new();

        for jwk in &self.published.keys {
            declared_algorithm(jwk)?;
            DecodingKey::from_jwk(jwk)?;
            loaded.push(declared_key_id(jwk)?.clone());
        }

        Ok(loaded)
    }
}

#[cfg(test)]
mod tests {
    use crate::consumer_claims::ConsumerClaims;
    use crate::consumer_expected_claims::consumer_expected_claims;

    use super::StandardVerifier;

    const P256_COORDINATES: &str = r#""crv":"P-256","kty":"EC","x":"MKBCTNIcKUSDii11ySs3526iDZ8AiTo7Tu6KPAqv7D4","y":"4Etl6SRW2YiLUrN5vfvVHuhp7x8PxltmWWlbbM4IFGI""#;
    const UNUSABLE_P256_COORDINATES: &str =
        r#""crv":"P-256","kty":"EC","x":"not base64url","y":"not base64url""#;
    const TOKEN_FOR_KID_A: &str = "eyJhbGciOiJFUzI1NiIsImtpZCI6ImEifQ.e30.c2ln";

    fn verifier(keys: &str) -> StandardVerifier {
        StandardVerifier::from_document(&format!(r#"{{"keys":[{keys}]}}"#))
            .expect("the fixture is a jwk set")
    }

    fn decode_failure(keys: &str, token: &str) -> String {
        verifier(keys)
            .decode::<ConsumerClaims>(token, &consumer_expected_claims())
            .expect_err("the token is not verifiable")
            .to_string()
    }

    fn load_failure(keys: &str) -> String {
        verifier(keys)
            .load_every_key()
            .expect_err("the published key is unusable")
            .to_string()
    }

    #[test]
    fn refuses_a_document_that_is_not_a_jwk_set() {
        assert!(StandardVerifier::from_document("not a jwk set").is_err());
    }

    #[test]
    fn refuses_a_published_key_without_an_algorithm() {
        assert_eq!(
            load_failure(&format!(r#"{{"kid":"a",{P256_COORDINATES}}}"#)),
            "the published key declares no alg"
        );
    }

    #[test]
    fn refuses_a_published_key_whose_algorithm_cannot_sign() {
        assert_eq!(
            load_failure(&format!(
                r#"{{"alg":"RSA-OAEP","kid":"a",{P256_COORDINATES}}}"#
            )),
            "the published alg 'RSA_OAEP' is not one margaret signs with"
        );
    }

    #[test]
    fn refuses_a_published_key_without_a_key_id() {
        assert_eq!(
            load_failure(&format!(r#"{{"alg":"ES256",{P256_COORDINATES}}}"#)),
            "the published key declares no kid"
        );
    }

    #[test]
    fn refuses_a_published_key_whose_coordinates_cannot_be_decoded() {
        assert!(
            load_failure(&format!(
                r#"{{"alg":"ES256","kid":"a",{UNUSABLE_P256_COORDINATES}}}"#
            ))
            .contains("Base64")
        );
    }

    #[test]
    fn refuses_a_token_whose_header_carries_no_key_id() {
        assert_eq!(
            decode_failure(
                &format!(r#"{{"alg":"ES256","kid":"a",{P256_COORDINATES}}}"#),
                "eyJhbGciOiJFUzI1NiJ9.e30.c2ln",
            ),
            "the token header carries no kid"
        );
    }

    #[test]
    fn refuses_a_token_whose_header_cannot_be_read() {
        assert!(!decode_failure(
            &format!(r#"{{"alg":"ES256","kid":"a",{P256_COORDINATES}}}"#),
            "not.a.token",
        )
        .is_empty());
    }

    #[test]
    fn refuses_a_token_signed_by_a_key_that_is_not_published() {
        assert_eq!(
            decode_failure(
                &format!(r#"{{"alg":"ES256","kid":"a",{P256_COORDINATES}}}"#),
                "eyJhbGciOiJFUzI1NiIsImtpZCI6InVucHVibGlzaGVkIn0.e30.c2ln",
            ),
            "the signing key 'unpublished' is not published"
        );
    }

    #[test]
    fn refuses_a_token_whose_signing_key_declares_no_algorithm() {
        assert_eq!(
            decode_failure(
                &format!(r#"{{"kid":"a",{P256_COORDINATES}}}"#),
                TOKEN_FOR_KID_A,
            ),
            "the published key declares no alg"
        );
    }

    #[test]
    fn refuses_a_token_whose_signing_key_cannot_be_decoded() {
        assert!(
            decode_failure(
                &format!(r#"{{"alg":"ES256","kid":"a",{UNUSABLE_P256_COORDINATES}}}"#),
                TOKEN_FOR_KID_A,
            )
            .contains("Base64")
        );
    }
}
