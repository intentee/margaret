use serde_json::Value;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::key_id::KeyId;
use margaret_jws_verification::verification_key::VerificationKey;

use crate::compact_jws::compact_jws;
use crate::ec_signing_key::EcSigningKey;
use crate::encoded_header::encoded_header;
use crate::jwks_key_error::JwksKeyError;
use crate::signing_input::signing_input;

#[derive(Clone)]
pub struct JwkPair {
    encoded_access_token_header: String,
    encoded_client_authentication_header: String,
    encoded_jwt_header: String,
    encoded_sign_in_transaction_header: String,
    kid: KeyId,
    public_jwk: Jwk,
    signing_key: EcSigningKey,
    verification_key: VerificationKey,
}

impl JwkPair {
    /// # Errors
    ///
    /// Returns `JwksKeyError::MissingPublicKeyCoordinate` when the public key cannot be published, and
    /// `JwksKeyError::VerificationKeyRejected` when its public key does not verify.
    pub fn new(kid: KeyId, signing_key: EcSigningKey) -> Result<Self, JwksKeyError> {
        let algorithm = signing_key.curve().curve().algorithm();

        signing_key.public_jwk(&kid).and_then(|public_jwk| {
            signing_key.verification_material().map(|material| {
                let verification_key = VerificationKey::new(kid.clone(), material);

                Self {
                    encoded_access_token_header: encoded_header(
                        algorithm,
                        &kid,
                        JwtType::AccessToken,
                    ),
                    encoded_client_authentication_header: encoded_header(
                        algorithm,
                        &kid,
                        JwtType::ClientAuthentication,
                    ),
                    encoded_jwt_header: encoded_header(algorithm, &kid, JwtType::Jwt),
                    encoded_sign_in_transaction_header: encoded_header(
                        algorithm,
                        &kid,
                        JwtType::SignInTransaction,
                    ),
                    kid,
                    public_jwk,
                    signing_key,
                    verification_key,
                }
            })
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

    #[must_use]
    pub fn sign_json(&self, claims: &Value, jwt_type: JwtType) -> String {
        let encoded_header = match jwt_type {
            JwtType::AccessToken => &self.encoded_access_token_header,
            JwtType::ClientAuthentication => &self.encoded_client_authentication_header,
            JwtType::Jwt => &self.encoded_jwt_header,
            JwtType::SignInTransaction => &self.encoded_sign_in_transaction_header,
        };
        let signing_input = signing_input(encoded_header, claims);
        let signature = self.signing_key.sign(signing_input.as_bytes());

        compact_jws(&signing_input, &signature)
    }

    #[must_use]
    pub fn signing_key(&self) -> &EcSigningKey {
        &self.signing_key
    }

    #[must_use]
    pub fn verification_key(&self) -> &VerificationKey {
        &self.verification_key
    }
}
