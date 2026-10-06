use chrono::Utc;
use serde_json::Value;
use serde_json::json;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret::JwksSecret;

#[must_use]
pub fn signed_by(
    secret: &JwksSecret,
    issuer: &str,
    audience: &Value,
    repository: &str,
    jwt_type: JwtType,
) -> String {
    let now = Utc::now().timestamp();

    secret.current().sign_json(
        &json!({
            "aud": audience,
            "exp": now + 300,
            "iat": now,
            "iss": issuer,
            "repository": repository,
            "sub": "repo:intentee/margaret:ref:refs/heads/main",
        }),
        jwt_type,
    )
}
