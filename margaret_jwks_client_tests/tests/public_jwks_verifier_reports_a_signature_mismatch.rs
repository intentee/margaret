use std::sync::Arc;

use margaret_jwks_client::access_token_verification::AccessTokenVerification;
use margaret_jwks_client::public_jwks_holder::PublicJwksHolder;
use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
use margaret_jwks_client_tests::test_claims::TestClaims;
use margaret_jwks_client_tests::test_claims_expiring_at::test_claims_expiring_at;
use margaret_jwks_client_tests::test_expected_claims::test_expected_claims;
use margaret_jwks_client_tests::test_instant::test_instant;
use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::generate_keypair::generate_keypair;
use margaret_jwks_keygen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::signs_claims::SignsClaims as _;

#[tokio::test]
async fn public_jwks_verifier_reports_a_signature_mismatch() {
    let kid = "shared-kid".to_string();
    let signing_keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: kid.clone(),
    })
    .expect("the signing keypair generates");
    let published_keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid,
    })
    .expect("the published keypair generates");
    let token = signing_keypair
        .signing
        .sign(&test_claims_expiring_at(2_000_000_000))
        .await
        .expect("the claims are signed");
    let holder = PublicJwksHolder::default();

    holder.set(Some(Arc::new(PublicJwks {
        keys: vec![published_keypair.public],
    })));

    let verification = PublicJwksVerifier::new(test_expected_claims(), holder)
        .verify::<TestClaims>(&token, test_instant(1_700_000_000))
        .expect("the published jwks is usable");

    assert!(matches!(
        verification,
        AccessTokenVerification::SignatureMismatch
    ));
}
