use std::sync::Arc;

use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::encode;
use margaret_macros::constructor;
use margaret_macros::singleton;

use crate::config::Config;
use crate::security::access_token_claims::AccessTokenClaims;
use crate::security::signs_token_claims::SignsTokenClaims;

#[singleton(provides = SignsTokenClaims)]
pub struct HmacTokenSigner {
    key: EncodingKey,
}

impl HmacTokenSigner {
    #[constructor]
    pub fn create(config: Arc<Config>) -> Self {
        Self {
            key: EncodingKey::from_secret(config.token_secret().as_bytes()),
        }
    }
}

impl SignsTokenClaims for HmacTokenSigner {
    fn sign(&self, claims: &AccessTokenClaims) -> String {
        encode(&Header::default(), claims, &self.key).expect("HS256 signing succeeds")
    }
}
