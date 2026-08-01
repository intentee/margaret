use std::error::Error;
use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use margaret_identity_session::is_expired::IsExpired;
use margaret_jwks_client::jwks_client_error::JwksClientError;
use margaret_jwks_client::public_jwks_holder::PublicJwksHolder;
use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
use margaret_jwks_client_tests::test_audience::TEST_AUDIENCE;
use margaret_jwks_client_tests::test_expected_claims::test_expected_claims;
use margaret_jwks_client_tests::test_instant::test_instant;
use margaret_jwks_client_tests::test_issuer::TEST_ISSUER;
use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::signs_claims::SignsClaims as _;
use margaret_jwt_claims::audience::Audience;
use margaret_jwt_claims::has_audience::HasAudience;
use margaret_jwt_claims::has_issuer::HasIssuer;

#[derive(Deserialize, Serialize)]
struct FailingExpiryClaims {
    aud: Audience,
    exp: i64,
    iss: String,
}

impl HasAudience for FailingExpiryClaims {
    fn audience(&self) -> &Audience {
        &self.aud
    }
}

impl HasIssuer for FailingExpiryClaims {
    fn issuer(&self) -> &str {
        &self.iss
    }
}

impl IsExpired for FailingExpiryClaims {
    fn is_expired(&self, _now: DateTime<Utc>) -> anyhow::Result<bool> {
        Err(anyhow::anyhow!("clock unavailable"))
    }
}

#[tokio::test]
async fn public_jwks_verifier_reports_an_expiry_system_error() {
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let token = secret
        .current
        .signing
        .sign(&FailingExpiryClaims {
            aud: Audience::One(TEST_AUDIENCE.to_string()),
            exp: 1_700_000_060,
            iss: TEST_ISSUER.to_string(),
        })
        .await
        .expect("the claims sign");
    let holder = PublicJwksHolder::default();
    holder.set(Some(Arc::new(PublicJwks::from(secret))));

    let error = PublicJwksVerifier::new(test_expected_claims(), holder)
        .verify::<FailingExpiryClaims>(&token, test_instant(1_700_000_000))
        .err()
        .expect("an unavailable expiry source is a system error");

    assert!(matches!(error, JwksClientError::TokenExpiry { .. }));
    assert_eq!(
        error
            .source()
            .expect("the expiry error is preserved")
            .to_string(),
        "clock unavailable"
    );
}
