use aws_lc_rs::signature::Ed25519KeyPair;
use aws_lc_rs::signature::KeyPair;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use serde_json::Value;
use serde_json::json;

use crate::signed_token::signed_token;
use crate::signing_input::signing_input;

pub struct FixtureOctetKeyPair {
    key_pair: Ed25519KeyPair,
    kid: String,
}

impl FixtureOctetKeyPair {
    /// # Panics
    ///
    /// Panics when the fixture key cannot be generated.
    #[must_use]
    pub fn generate(kid: &str) -> Self {
        Self {
            key_pair: Ed25519KeyPair::generate().expect("the fixture key generates"),
            kid: kid.to_string(),
        }
    }

    #[must_use]
    pub fn jwk(&self, alg: &str) -> Value {
        json!({
            "alg": alg,
            "crv": "Ed25519",
            "kid": self.kid,
            "kty": "OKP",
            "use": "sig",
            "x": Base64UrlUnpadded::encode_string(self.key_pair.public_key().as_ref()),
        })
    }

    #[must_use]
    pub fn token(&self, alg: &str, claims: &Value) -> String {
        let signing_input = signing_input(&json!({ "alg": alg, "kid": self.kid }), claims);

        signed_token(
            &signing_input,
            self.key_pair.sign(signing_input.as_bytes()).as_ref(),
        )
    }
}
