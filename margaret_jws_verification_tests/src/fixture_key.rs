use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use p256::ecdsa::signature::Signer;
use p256::elliptic_curve::rand_core::OsRng;
use serde_json::Value;
use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::key_use::KeyUse;
use margaret_jws_verification::ec_jwk::EcJwk;
use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::key_id::KeyId;

use crate::fixture_material::FixtureMaterial;

fn encoded(coordinate: Option<&[u8]>) -> String {
    Base64UrlUnpadded::encode_string(
        coordinate.expect("an uncompressed point has both coordinates"),
    )
}

pub struct FixtureKey {
    kid: String,
    material: FixtureMaterial,
}

impl FixtureKey {
    #[must_use]
    pub fn generate(curve: Curve, kid: &str) -> Self {
        let material = match curve {
            Curve::P256 => FixtureMaterial::P256(p256::ecdsa::SigningKey::random(&mut OsRng)),
            Curve::P384 => FixtureMaterial::P384(p384::ecdsa::SigningKey::random(&mut OsRng)),
        };

        Self {
            kid: kid.to_string(),
            material,
        }
    }

    #[must_use]
    pub fn ec_jwk(&self) -> EcJwk {
        match &self.material {
            FixtureMaterial::P256(key) => {
                let point = key.verifying_key().to_encoded_point(false);

                self.ec_jwk_of(
                    Curve::P256,
                    point.x().map(AsRef::as_ref),
                    point.y().map(AsRef::as_ref),
                )
            }
            FixtureMaterial::P384(key) => {
                let point = key.verifying_key().to_encoded_point(false);

                self.ec_jwk_of(
                    Curve::P384,
                    point.x().map(AsRef::as_ref),
                    point.y().map(AsRef::as_ref),
                )
            }
        }
    }

    #[must_use]
    pub fn header(&self) -> Value {
        json!({ "alg": self.ec_jwk().crv.algorithm().wire_name(), "kid": self.kid })
    }

    #[must_use]
    pub fn jwk(&self) -> Jwk {
        Jwk::Ec(self.ec_jwk())
    }

    #[must_use]
    pub fn signature(&self, signing_input: &str) -> Vec<u8> {
        match &self.material {
            FixtureMaterial::P256(key) => {
                let signature: p256::ecdsa::Signature = key.sign(signing_input.as_bytes());

                signature.to_bytes().to_vec()
            }
            FixtureMaterial::P384(key) => {
                let signature: p384::ecdsa::Signature = key.sign(signing_input.as_bytes());

                signature.to_bytes().to_vec()
            }
        }
    }

    #[must_use]
    pub fn signing_input(header: &Value, claims: &Value) -> String {
        format!(
            "{}.{}",
            Base64UrlUnpadded::encode_string(header.to_string().as_bytes()),
            Base64UrlUnpadded::encode_string(claims.to_string().as_bytes()),
        )
    }

    #[must_use]
    pub fn token(&self, header: &Value, claims: &Value) -> String {
        let signing_input = Self::signing_input(header, claims);
        let signature = Base64UrlUnpadded::encode_string(&self.signature(&signing_input));

        format!("{signing_input}.{signature}")
    }

    fn ec_jwk_of(&self, curve: Curve, x: Option<&[u8]>, y: Option<&[u8]>) -> EcJwk {
        EcJwk {
            alg: Some(curve.algorithm()),
            crv: curve,
            kid: Some(KeyId::new(self.kid.clone())),
            key_use: Some(KeyUse::Signature),
            x: encoded(x),
            y: encoded(y),
        }
    }
}
