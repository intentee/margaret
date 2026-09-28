use std::error::Error;
use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use margaret_identity_session::is_expired::IsExpired;
use margaret_jose_parameters::curve::Curve;
use margaret_jwks_client::jwks_client_error::JwksClientError;
use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
use margaret_jwks_client::verification_key_set_holder::VerificationKeySetHolder;
use margaret_jwks_client_tests::test_instant::test_instant;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signs_claims::SignsClaims as _;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;

#[derive(Deserialize, Serialize)]
struct FailingExpiryClaims {
    exp: i64,
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
        .current()
        .sign(&FailingExpiryClaims { exp: 1_700_000_060 })
        .await
        .expect("the claims sign");
    let KeySetParsing::Accepted(key_set) =
        VerificationKeySet::from_jwks(secret.public_jwks().keys().to_vec())
    else {
        panic!("the published key set is accepted");
    };
    let holder = VerificationKeySetHolder::default();

    holder.set(Some(Arc::new(key_set)));

    let error = PublicJwksVerifier::new(holder)
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
