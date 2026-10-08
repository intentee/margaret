use serde_json::Value;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::key_id::KeyId;
use margaret_jws_verification::verification_key::VerificationKey;

use crate::compact_jws::compact_jws;
use crate::encoded_header::encoded_header;
use crate::jwks_key_error::JwksKeyError;
use crate::rsa_signing_key::RsaSigningKey;
use crate::signing_input::signing_input;

#[derive(Clone)]
pub struct RsaJwkPair {
    encoded_jwt_header: String,
    kid: KeyId,
    public_jwk: Jwk,
    signing_key: RsaSigningKey,
    verification_key: VerificationKey,
}

impl RsaJwkPair {
    /// # Errors
    ///
    /// Returns `JwksKeyError::RsaVerificationKeyRejected` when the public key does not verify.
    pub fn new(kid: KeyId, signing_key: RsaSigningKey) -> Result<Self, JwksKeyError> {
        signing_key.verification_material().map(|material| Self {
            encoded_jwt_header: encoded_header(JwsAlgorithm::Rs256, &kid, JwtType::Jwt),
            public_jwk: signing_key.public_jwk(&kid),
            verification_key: VerificationKey::new(kid.clone(), material),
            kid,
            signing_key,
        })
    }

    #[must_use]
    pub fn kid(&self) -> &KeyId {
        &self.kid
    }

    #[must_use]
    pub fn public_jwk(&self) -> &Jwk {
        &self.public_jwk
    }

    /// # Errors
    ///
    /// Returns `JwksKeyError::RsaSigning` when the signing key cannot sign the claims.
    pub fn sign_jwt(&self, claims: &Value) -> Result<String, JwksKeyError> {
        let signing_input = signing_input(&self.encoded_jwt_header, claims);

        self.signing_key
            .sign(signing_input.as_bytes())
            .map(|signature| compact_jws(&signing_input, &signature))
    }

    #[must_use]
    pub fn signing_key(&self) -> &RsaSigningKey {
        &self.signing_key
    }

    #[must_use]
    pub fn verification_key(&self) -> &VerificationKey {
        &self.verification_key
    }
}
