use std::ops::ControlFlow;

use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use p256::ecdsa::signature::Signer;
use p256::elliptic_curve::rand_core::OsRng;
use p256::pkcs8;
use p256::pkcs8::DecodePrivateKey;
use p256::pkcs8::EncodePrivateKey;
use p256::pkcs8::SecretDocument;
use zeroize::Zeroizing;

use margaret_jose_parameters::key_use::KeyUse;
use margaret_jws_verification::ec_jwk::EcJwk;
use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::key_id::KeyId;
use margaret_jws_verification::verification_material::VerificationMaterial;

use crate::jwks_key_error::JwksKeyError;
use crate::signing_curve::SigningCurve;
use crate::signing_material::SigningMaterial;

fn encoded_coordinate(
    coordinate: Option<&[u8]>,
    name: &'static str,
) -> Result<String, JwksKeyError> {
    match coordinate {
        Some(bytes) => Ok(Base64UrlUnpadded::encode_string(bytes)),
        None => Err(JwksKeyError::MissingPublicKeyCoordinate { coordinate: name }),
    }
}

fn ec_jwk(
    curve: SigningCurve,
    kid: &KeyId,
    x: Option<&[u8]>,
    y: Option<&[u8]>,
) -> Result<Jwk, JwksKeyError> {
    Ok(Jwk::Ec(EcJwk {
        alg: Some(curve.curve().algorithm()),
        crv: curve.curve(),
        kid: Some(kid.clone()),
        key_use: Some(KeyUse::Signature),
        x: encoded_coordinate(x, "x")?,
        y: encoded_coordinate(y, "y")?,
    }))
}

fn ec_verification_material(
    curve: SigningCurve,
    point: &[u8],
) -> Result<VerificationMaterial, JwksKeyError> {
    match VerificationMaterial::from_ec_point(curve.curve(), point) {
        ControlFlow::Continue(material) => Ok(material),
        ControlFlow::Break(source) => Err(JwksKeyError::VerificationKeyRejected { source }),
    }
}

fn with_pkcs8(
    material: SigningMaterial,
    document: Result<SecretDocument, pkcs8::Error>,
) -> Result<EcSigningKey, JwksKeyError> {
    match document {
        Ok(document) => Ok(EcSigningKey {
            material,
            pkcs8: Zeroizing::new(document.as_bytes().to_vec()),
        }),
        Err(source) => Err(JwksKeyError::Pkcs8Encoding { source }),
    }
}

#[derive(Clone, PartialEq)]
pub struct EcSigningKey {
    material: SigningMaterial,
    pkcs8: Zeroizing<Vec<u8>>,
}

impl EcSigningKey {
    /// # Errors
    ///
    /// Returns `JwksKeyError::SigningKeyRejected` when the document is not a pkcs#8 key of the
    /// curve.
    pub fn from_pkcs8(
        curve: SigningCurve,
        pkcs8: Zeroizing<Vec<u8>>,
    ) -> Result<Self, JwksKeyError> {
        let material = match curve {
            SigningCurve::P256 => {
                p256::ecdsa::SigningKey::from_pkcs8_der(&pkcs8).map(SigningMaterial::P256)
            }
            SigningCurve::P384 => {
                p384::ecdsa::SigningKey::from_pkcs8_der(&pkcs8).map(SigningMaterial::P384)
            }
        };

        match material {
            Ok(material) => Ok(Self { material, pkcs8 }),
            Err(source) => Err(JwksKeyError::SigningKeyRejected { source }),
        }
    }

    /// # Errors
    ///
    /// Returns `JwksKeyError::Pkcs8Encoding` when the generated key cannot be encoded.
    pub fn generate(curve: SigningCurve) -> Result<Self, JwksKeyError> {
        match curve {
            SigningCurve::P256 => {
                let key = p256::ecdsa::SigningKey::random(&mut OsRng);
                let document = key.to_pkcs8_der();

                with_pkcs8(SigningMaterial::P256(key), document)
            }
            SigningCurve::P384 => {
                let key = p384::ecdsa::SigningKey::random(&mut OsRng);
                let document = key.to_pkcs8_der();

                with_pkcs8(SigningMaterial::P384(key), document)
            }
        }
    }

    #[must_use]
    pub fn curve(&self) -> SigningCurve {
        match self.material {
            SigningMaterial::P256(_) => SigningCurve::P256,
            SigningMaterial::P384(_) => SigningCurve::P384,
        }
    }

    #[must_use]
    pub fn pkcs8(&self) -> &Zeroizing<Vec<u8>> {
        &self.pkcs8
    }

    pub(crate) fn public_jwk(&self, kid: &KeyId) -> Result<Jwk, JwksKeyError> {
        match &self.material {
            SigningMaterial::P256(key) => {
                let point = key.verifying_key().to_encoded_point(false);

                ec_jwk(
                    SigningCurve::P256,
                    kid,
                    point.x().map(AsRef::as_ref),
                    point.y().map(AsRef::as_ref),
                )
            }
            SigningMaterial::P384(key) => {
                let point = key.verifying_key().to_encoded_point(false);

                ec_jwk(
                    SigningCurve::P384,
                    kid,
                    point.x().map(AsRef::as_ref),
                    point.y().map(AsRef::as_ref),
                )
            }
        }
    }

    pub(crate) fn sign(&self, message: &[u8]) -> Vec<u8> {
        match &self.material {
            SigningMaterial::P256(key) => {
                let signature: p256::ecdsa::Signature = key.sign(message);

                signature
                    .normalize_s()
                    .unwrap_or(signature)
                    .to_bytes()
                    .to_vec()
            }
            SigningMaterial::P384(key) => {
                let signature: p384::ecdsa::Signature = key.sign(message);

                signature
                    .normalize_s()
                    .unwrap_or(signature)
                    .to_bytes()
                    .to_vec()
            }
        }
    }

    pub(crate) fn verification_material(&self) -> Result<VerificationMaterial, JwksKeyError> {
        let point = match &self.material {
            SigningMaterial::P256(key) => key.verifying_key().to_encoded_point(false).to_bytes(),
            SigningMaterial::P384(key) => key.verifying_key().to_encoded_point(false).to_bytes(),
        };

        ec_verification_material(self.curve(), &point)
    }
}

#[cfg(test)]
mod tests {
    use p256::pkcs8::DecodePrivateKey;

    use margaret_jose_parameters::curve::Curve;
    use margaret_jws_verification::key_id::KeyId;
    use margaret_jws_verification::verification_material::VerificationMaterial;

    use super::EcSigningKey;
    use super::SigningMaterial;
    use super::ec_jwk;
    use super::ec_verification_material;
    use super::with_pkcs8;
    use crate::jwks_key_error::JwksKeyError;
    use crate::signing_curve::SigningCurve;

    fn kid() -> KeyId {
        KeyId::new("kid".to_string())
    }

    #[test]
    fn publishes_the_curve_of_a_generated_key() {
        let key = EcSigningKey::generate(SigningCurve::P384).expect("the key generates");

        let published = serde_json::to_value(key.public_jwk(&kid()).expect("the key publishes"))
            .expect("the published key serializes");

        assert_eq!(published["crv"], "P-384");
    }

    #[test]
    fn reports_a_key_that_cannot_be_encoded_to_pkcs8() {
        let encoding_failure = p256::ecdsa::SigningKey::from_pkcs8_der(&[])
            .expect_err("the fixture is not a pkcs#8 document");
        let key = p256::ecdsa::SigningKey::from_slice(&[1; 32]).expect("the scalar is a key");

        assert_eq!(
            with_pkcs8(SigningMaterial::P256(key), Err(encoding_failure))
                .map(drop)
                .expect_err("the encoding failure is reported")
                .to_string(),
            format!(
                "the generated private key could not be encoded to pkcs#8: {}",
                p256::ecdsa::SigningKey::from_pkcs8_der(&[])
                    .expect_err("the fixture is not a pkcs#8 document")
            )
        );
    }

    #[test]
    fn reports_a_point_without_an_x_coordinate() {
        assert_eq!(
            ec_jwk(SigningCurve::P256, &kid(), None, Some(&[2]))
                .expect_err("the missing coordinate is reported")
                .to_string(),
            "the public key is missing its x coordinate"
        );
    }

    #[test]
    fn reports_a_point_without_a_y_coordinate() {
        assert_eq!(
            ec_jwk(SigningCurve::P256, &kid(), Some(&[1]), None)
                .expect_err("the missing coordinate is reported")
                .to_string(),
            "the public key is missing its y coordinate"
        );
    }

    #[test]
    fn reports_a_point_that_is_not_on_the_curve() {
        let point = [4; 65];
        let rejection = VerificationMaterial::from_ec_point(Curve::P256, &point)
            .break_value()
            .expect("the point is not on the curve");

        assert!(matches!(
            ec_verification_material(SigningCurve::P256, &point),
            Err(JwksKeyError::VerificationKeyRejected { source }) if source == rejection
        ));
    }
}
