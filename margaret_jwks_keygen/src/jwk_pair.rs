use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use serde_json::Map;
use serde_json::Value;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::key_id::KeyId;

use crate::ec_signing_key::EcSigningKey;
use crate::jwks_key_error::JwksKeyError;

fn encoded_header(algorithm: JwsAlgorithm, kid: &KeyId, jwt_type: JwtType) -> String {
    let mut header = Map::new();

    header.insert(
        "alg".to_string(),
        Value::String(algorithm.wire_name().to_string()),
    );
    header.insert("kid".to_string(), Value::String(kid.as_str().to_string()));
    header.insert(
        "typ".to_string(),
        Value::String(jwt_type.wire_name().to_string()),
    );

    Base64UrlUnpadded::encode_string(Value::Object(header).to_string().as_bytes())
}

#[derive(Clone, PartialEq)]
pub struct JwkPair {
    encoded_access_token_header: String,
    encoded_jwt_header: String,
    kid: KeyId,
    public_jwk: Jwk,
    signing_key: EcSigningKey,
}

impl JwkPair {
    /// # Errors
    ///
    /// Returns `JwksKeyError::MissingPublicKeyCoordinate` when the public key cannot be published.
    pub fn new(kid: KeyId, signing_key: EcSigningKey) -> Result<Self, JwksKeyError> {
        let algorithm = signing_key.curve().algorithm();

        signing_key.public_jwk(&kid).map(|public_jwk| Self {
            encoded_access_token_header: encoded_header(algorithm, &kid, JwtType::AccessToken),
            encoded_jwt_header: encoded_header(algorithm, &kid, JwtType::Jwt),
            kid,
            public_jwk,
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

    #[must_use]
    pub fn sign_json(&self, claims: &Value, jwt_type: JwtType) -> String {
        let encoded_header = match jwt_type {
            JwtType::AccessToken => &self.encoded_access_token_header,
            JwtType::Jwt => &self.encoded_jwt_header,
        };
        let signing_input = format!(
            "{encoded_header}.{}",
            Base64UrlUnpadded::encode_string(claims.to_string().as_bytes()),
        );
        let signature =
            Base64UrlUnpadded::encode_string(&self.signing_key.sign(signing_input.as_bytes()));

        format!("{signing_input}.{signature}")
    }

    #[must_use]
    pub fn signing_key(&self) -> &EcSigningKey {
        &self.signing_key
    }
}
