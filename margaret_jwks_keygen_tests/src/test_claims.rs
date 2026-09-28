use serde::Deserialize;
use serde_json::json;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwk_pair::JwkPair;

use crate::far_future_expiry::FAR_FUTURE_EXPIRY;

#[derive(Debug, Deserialize, PartialEq)]
pub struct TestClaims {
    pub sub: String,
}

impl TestClaims {
    #[must_use]
    pub fn signed_by(&self, pair: &JwkPair) -> String {
        pair.sign_json(
            &json!({ "exp": FAR_FUTURE_EXPIRY, "iat": 0, "sub": self.sub }),
            JwtType::AccessToken,
        )
    }
}
