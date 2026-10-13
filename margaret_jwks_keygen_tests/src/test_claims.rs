use serde::Deserialize;
use serde_json::json;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwk_pair::JwkPair;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;

use crate::far_future_expiry::FAR_FUTURE_EXPIRY;

#[derive(Debug, Deserialize, PartialEq)]
pub struct TestClaims {
    pub sub: String,
}

impl TestClaims {
    #[must_use]
    pub fn signed_by(&self, pair: &JwkPair) -> String {
        let trust = fixture_trust();

        pair.sign_json(
            &json!({
                "aud": trust.audience,
                "exp": FAR_FUTURE_EXPIRY,
                "iat": 0,
                "iss": trust.issuer,
                "sub": self.sub,
            }),
            JwtType::AccessToken,
        )
    }
}
