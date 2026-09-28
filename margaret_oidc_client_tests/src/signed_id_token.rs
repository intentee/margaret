use serde_json::Value;
use serde_json::json;

use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;
use margaret_token_trust::token_trust::TokenTrust;

pub struct SignedIdToken {
    pub audience: Value,
    pub exp: i64,
    pub typ: &'static str,
}

impl SignedIdToken {
    #[must_use]
    pub fn signed_by(&self, key: &FixtureRsaKey, trust: &TokenTrust) -> String {
        let mut header = key.header();

        header["typ"] = json!(self.typ);

        key.token(
            &header,
            &json!({
                "aud": self.audience,
                "exp": self.exp,
                "iat": 1_700_000_000,
                "iss": trust.issuer.as_str(),
                "role": "builder",
                "sub": "service-account",
            }),
        )
    }
}
