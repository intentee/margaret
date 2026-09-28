use aws_lc_rs::rand::SystemRandom;
use aws_lc_rs::signature::KeyPair;
use aws_lc_rs::signature::RSA_PKCS1_SHA256;
use aws_lc_rs::signature::RsaKeyPair;
use aws_lc_rs::signature::RsaPublicKeyComponents;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use serde_json::Value;
use serde_json::json;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jose_parameters::key_use::KeyUse;
use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::key_id::KeyId;
use margaret_jws_verification::rsa_jwk::RsaJwk;

use crate::signed_token::signed_token;
use crate::signing_input::signing_input;

pub struct FixtureRsaKey {
    key_pair: RsaKeyPair,
    kid: String,
}

impl FixtureRsaKey {
    #[must_use]
    /// # Panics
    ///
    /// Panics when the committed fixture is not a pkcs#8 rsa key.
    pub fn load(kid: &str) -> Self {
        Self {
            key_pair: RsaKeyPair::from_pkcs8(include_bytes!("../fixtures/rsa_2048_pkcs8.der"))
                .expect("the committed fixture is a pkcs#8 rsa key"),
            kid: kid.to_string(),
        }
    }

    #[must_use]
    pub fn header(&self) -> Value {
        json!({ "alg": JwsAlgorithm::Rs256.wire_name(), "kid": self.kid })
    }

    #[must_use]
    pub fn jwk(&self) -> Jwk {
        Jwk::Rsa(self.rsa_jwk())
    }

    #[must_use]
    pub fn rsa_jwk(&self) -> RsaJwk {
        let RsaPublicKeyComponents { n, e } =
            RsaPublicKeyComponents::<Vec<u8>>::from(self.key_pair.public_key());

        RsaJwk {
            alg: Some(JwsAlgorithm::Rs256),
            e: Base64UrlUnpadded::encode_string(&e),
            kid: Some(KeyId::new(self.kid.clone())),
            key_use: Some(KeyUse::Signature),
            n: Base64UrlUnpadded::encode_string(&n),
        }
    }

    #[must_use]
    /// # Panics
    ///
    /// Panics when the fixture key cannot sign.
    pub fn signature(&self, signing_input: &str) -> Vec<u8> {
        let mut signature = vec![0; self.key_pair.public_modulus_len()];

        self.key_pair
            .sign(
                &RSA_PKCS1_SHA256,
                &SystemRandom::new(),
                signing_input.as_bytes(),
                &mut signature,
            )
            .expect("the fixture key signs");

        signature
    }

    #[must_use]
    pub fn token(&self, header: &Value, claims: &Value) -> String {
        let signing_input = signing_input(header, claims);

        signed_token(&signing_input, &self.signature(&signing_input))
    }
}
