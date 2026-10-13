use serde_json::json;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

use crate::access_token_claims::access_token_claims;

#[must_use]
pub fn kid_less_access_token(key: &FixtureKey) -> String {
    key.token(
        &json!({
            "alg": key.header()["alg"],
            "typ": JwtType::AccessToken.wire_name(),
        }),
        &access_token_claims(),
    )
}
