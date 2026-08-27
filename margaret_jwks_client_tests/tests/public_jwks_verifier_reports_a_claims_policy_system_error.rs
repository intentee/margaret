use std::error::Error;
use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use margaret_identity_session::accepts_claims::AcceptsClaims;
use margaret_identity_session::claims_acceptance::ClaimsAcceptance;
use margaret_jwks_client::jwks_client_error::JwksClientError;
use margaret_jwks_client::public_jwks_holder::PublicJwksHolder;
use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
use margaret_jwks_client_tests::test_instant::test_instant;
use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::signs_claims::SignsClaims as _;

#[derive(Deserialize, Serialize)]
struct FailingPolicyClaims {
    exp: i64,
}

impl AcceptsClaims for FailingPolicyClaims {
    fn accepts(&self, _now: DateTime<Utc>) -> anyhow::Result<ClaimsAcceptance> {
        Err(anyhow::anyhow!("clock unavailable"))
    }
}

#[tokio::test]
async fn public_jwks_verifier_reports_a_claims_policy_system_error() {
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let token = secret
        .current
        .signing
        .sign(&FailingPolicyClaims { exp: 1_700_000_060 })
        .await
        .expect("the claims sign");
    let holder = PublicJwksHolder::default();
    holder.set(Some(Arc::new(PublicJwks::from(secret))));

    let error = PublicJwksVerifier::new(holder)
        .verify::<FailingPolicyClaims>(&token, test_instant(1_700_000_000))
        .err()
        .expect("an unavailable claims policy is a system error");

    assert!(matches!(error, JwksClientError::ClaimsAcceptance { .. }));
    assert_eq!(
        error
            .source()
            .expect("the claims policy error is preserved")
            .to_string(),
        "clock unavailable"
    );
}
