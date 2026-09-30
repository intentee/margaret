use aws_lc_rs::rand::SystemRandom;
use aws_lc_rs::signature::ECDSA_P256_SHA256_FIXED_SIGNING;
use aws_lc_rs::signature::ECDSA_P384_SHA384_FIXED_SIGNING;
use aws_lc_rs::signature::ECDSA_P521_SHA512_FIXED_SIGNING;
use aws_lc_rs::signature::EcdsaKeyPair;
use aws_lc_rs::signature::EcdsaSigningAlgorithm;
use aws_lc_rs::signature::KeyPair;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use serde_json::Value;
use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::key_use::KeyUse;
use margaret_jws_verification::ec_jwk::EcJwk;
use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::key_id::KeyId;
use margaret_jws_verification::verification_key::VerificationKey;
use margaret_jws_verification::verification_material::VerificationMaterial;

use crate::fixture_certificate::fixture_certificate;
use crate::signed_token::signed_token;
use crate::signing_input::signing_input;

fn signing_algorithm(curve: Curve) -> &'static EcdsaSigningAlgorithm {
    match curve {
        Curve::P256 => &ECDSA_P256_SHA256_FIXED_SIGNING,
        Curve::P384 => &ECDSA_P384_SHA384_FIXED_SIGNING,
        Curve::P521 => &ECDSA_P521_SHA512_FIXED_SIGNING,
    }
}

pub struct FixtureKey {
    curve: Curve,
    key_pair: EcdsaKeyPair,
    kid: String,
}

impl FixtureKey {
    /// # Panics
    ///
    /// Panics when the fixture key cannot be generated.
    #[must_use]
    pub fn generate(curve: Curve, kid: &str) -> Self {
        Self {
            curve,
            key_pair: EcdsaKeyPair::generate(signing_algorithm(curve))
                .expect("the fixture key generates"),
            kid: kid.to_string(),
        }
    }

    /// # Panics
    ///
    /// Panics when the fixture key cannot be encoded or certified.
    #[must_use]
    pub fn certificate(&self) -> Vec<u8> {
        fixture_certificate(
            self.key_pair
                .to_pkcs8v1()
                .expect("the fixture key encodes as pkcs#8")
                .as_ref(),
        )
    }

    #[must_use]
    pub fn ec_jwk(&self) -> EcJwk {
        let coordinates = &self.key_pair.public_key().as_ref()[1..];
        let coordinate_octets = self.curve.coordinate_octets();

        EcJwk {
            alg: Some(self.curve.algorithm()),
            crv: self.curve,
            kid: Some(KeyId::new(self.kid.clone())),
            key_use: Some(KeyUse::Signature),
            x: Base64UrlUnpadded::encode_string(&coordinates[..coordinate_octets]),
            y: Base64UrlUnpadded::encode_string(&coordinates[coordinate_octets..]),
        }
    }

    #[must_use]
    pub fn header(&self) -> Value {
        json!({ "alg": self.curve.algorithm().wire_name(), "kid": self.kid })
    }

    #[must_use]
    pub fn jwk(&self) -> Jwk {
        Jwk::Ec(self.ec_jwk())
    }

    /// # Panics
    ///
    /// Panics when the fixture key cannot sign.
    #[must_use]
    pub fn signature(&self, signing_input: &str) -> Vec<u8> {
        self.key_pair
            .sign(&SystemRandom::new(), signing_input.as_bytes())
            .expect("the fixture key signs")
            .as_ref()
            .to_vec()
    }

    #[must_use]
    pub fn token(&self, header: &Value, claims: &Value) -> String {
        let signing_input = signing_input(header, claims);

        signed_token(&signing_input, &self.signature(&signing_input))
    }

    /// # Panics
    ///
    /// Panics when the fixture public key does not verify.
    #[must_use]
    pub fn verification_key(&self) -> VerificationKey {
        VerificationKey::new(
            KeyId::new(self.kid.clone()),
            VerificationMaterial::from_ec_point(self.curve, self.key_pair.public_key().as_ref())
                .expect("the fixture public key verifies"),
        )
    }
}
