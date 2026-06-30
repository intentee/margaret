use std::sync::Arc;

use jsonwebtoken::Algorithm;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use margaret_macros::constructor;
use margaret_macros::singleton;

use crate::config::Config;
use crate::security::access_token_claims::AccessTokenClaims;
use crate::security::verifies_token::VerifiesToken;

#[singleton(provides = VerifiesToken)]
pub struct HmacTokenVerifier {
    key: DecodingKey,
    validation: Validation,
}

impl HmacTokenVerifier {
    #[constructor]
    pub fn create(config: Arc<Config>) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);

        validation.validate_aud = false;

        Self {
            key: DecodingKey::from_secret(config.token_secret().as_bytes()),
            validation,
        }
    }
}

impl VerifiesToken for HmacTokenVerifier {
    fn verify(&self, token: &str) -> Option<AccessTokenClaims> {
        decode::<AccessTokenClaims>(token, &self.key, &self.validation)
            .ok()
            .map(|token_data| token_data.claims)
    }
}
