use std::sync::Arc;

use aws_lc_rs::encoding::AsDer;
use aws_lc_rs::encoding::Pkcs8V1Der;
use aws_lc_rs::error::Unspecified;
use aws_lc_rs::rand::SystemRandom;
use aws_lc_rs::rsa::KeySize;
use aws_lc_rs::signature::KeyPair;
use aws_lc_rs::signature::RSA_PKCS1_SHA256;
use aws_lc_rs::signature::RsaKeyPair;
use aws_lc_rs::signature::RsaPublicKeyComponents;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use zeroize::Zeroizing;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jose_parameters::key_use::KeyUse;
use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::key_id::KeyId;
use margaret_jws_verification::rsa_jwk::RsaJwk;

use crate::jwks_key_error::JwksKeyError;

fn pkcs8_of(
    document: Result<Pkcs8V1Der<'static>, Unspecified>,
) -> Result<Zeroizing<Vec<u8>>, JwksKeyError> {
    document
        .map(|document| Zeroizing::new(document.as_ref().to_vec()))
        .map_err(|source| JwksKeyError::RsaKeyEncoding { source })
}

fn generated(key_pair: Result<RsaKeyPair, Unspecified>) -> Result<RsaSigningKey, JwksKeyError> {
    key_pair
        .map_err(|source| JwksKeyError::RsaKeyGeneration { source })
        .and_then(|key_pair| {
            pkcs8_of(AsDer::<Pkcs8V1Der<'static>>::as_der(&key_pair)).map(|pkcs8| RsaSigningKey {
                key_pair: Arc::new(key_pair),
                pkcs8,
            })
        })
}

fn signed(signature: Vec<u8>, outcome: Result<(), Unspecified>) -> Result<Vec<u8>, JwksKeyError> {
    match outcome {
        Ok(()) => Ok(signature),
        Err(source) => Err(JwksKeyError::RsaSigning { source }),
    }
}

#[derive(Clone)]
pub struct RsaSigningKey {
    key_pair: Arc<RsaKeyPair>,
    pkcs8: Zeroizing<Vec<u8>>,
}

impl RsaSigningKey {
    /// # Errors
    ///
    /// Returns `JwksKeyError::RsaKeyRejected` when the document is not a pkcs#8 rsa key.
    pub fn from_pkcs8(pkcs8: Zeroizing<Vec<u8>>) -> Result<Self, JwksKeyError> {
        match RsaKeyPair::from_pkcs8(&pkcs8) {
            Ok(key_pair) => Ok(Self {
                key_pair: Arc::new(key_pair),
                pkcs8,
            }),
            Err(source) => Err(JwksKeyError::RsaKeyRejected { source }),
        }
    }

    /// # Errors
    ///
    /// Returns `JwksKeyError::RsaKeyGeneration` when the key cannot be generated, and
    /// `JwksKeyError::RsaKeyEncoding` when it cannot be encoded to pkcs#8.
    pub fn generate() -> Result<Self, JwksKeyError> {
        generated(RsaKeyPair::generate(KeySize::Rsa2048))
    }

    #[must_use]
    pub fn pkcs8(&self) -> &Zeroizing<Vec<u8>> {
        &self.pkcs8
    }

    pub(crate) fn public_jwk(&self, kid: &KeyId) -> Jwk {
        let RsaPublicKeyComponents { n, e } =
            RsaPublicKeyComponents::<Vec<u8>>::from(self.key_pair.public_key());

        Jwk::Rsa(RsaJwk {
            alg: Some(JwsAlgorithm::Rs256),
            e: Base64UrlUnpadded::encode_string(&e),
            kid: Some(kid.clone()),
            key_use: Some(KeyUse::Signature),
            n: Base64UrlUnpadded::encode_string(&n),
        })
    }

    pub(crate) fn sign(&self, message: &[u8]) -> Result<Vec<u8>, JwksKeyError> {
        let mut signature = vec![0; self.key_pair.public_modulus_len()];
        let outcome = self.key_pair.sign(
            &RSA_PKCS1_SHA256,
            &SystemRandom::new(),
            message,
            &mut signature,
        );

        signed(signature, outcome)
    }
}

#[cfg(test)]
mod tests {
    use aws_lc_rs::error::Unspecified;

    use super::generated;
    use super::pkcs8_of;
    use super::signed;

    #[test]
    fn reports_a_key_that_cannot_be_generated() {
        assert_eq!(
            generated(Err(Unspecified))
                .map(drop)
                .expect_err("the generation failure is reported")
                .to_string(),
            format!("the rsa signing key could not be generated: {Unspecified}")
        );
    }

    #[test]
    fn reports_a_key_that_cannot_be_encoded() {
        assert_eq!(
            pkcs8_of(Err(Unspecified))
                .map(drop)
                .expect_err("the encoding failure is reported")
                .to_string(),
            format!("the rsa signing key could not be encoded to pkcs#8: {Unspecified}")
        );
    }

    #[test]
    fn reports_a_signature_that_cannot_be_produced() {
        assert_eq!(
            signed(Vec::new(), Err(Unspecified))
                .expect_err("the signing failure is reported")
                .to_string(),
            format!("the rsa signing key could not sign: {Unspecified}")
        );
    }
}
