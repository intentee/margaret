use std::sync::Arc;

use margaret_jwks_client::access_token_verification::AccessTokenVerification;
use margaret_jwks_client::public_jwks_holder::PublicJwksHolder;
use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
use margaret_jwks_client_tests::test_audience::TEST_AUDIENCE;
use margaret_jwks_client_tests::test_claims::TestClaims;
use margaret_jwks_client_tests::test_instant::test_instant;
use margaret_jwks_client_tests::test_issuer::TEST_ISSUER;
use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::generate_keypair::generate_keypair;
use margaret_jwks_keygen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_keygen::jwk_public::JwkPublic;
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
        .sign(&TestClaims {
            aud: TEST_AUDIENCE.to_string(),
            exp: 2_000_000_000,
            iss: TEST_ISSUER.to_string(),
            nbf: 0,
            sub: "subject".to_string(),
        })
        .await
        .expect("the claims are signed");
    let holder = PublicJwksHolder::default();

    holder.set(Some(Arc::new(
        PublicJwks::new(vec![JwkPublic::Ec(published_keypair.public)])
            .expect("the key set has unique key ids"),
    )));

    let verification = PublicJwksVerifier::new(holder)
        .verify::<TestClaims>(&token, test_instant(1_700_000_000))
        .expect("the published jwks is usable");

    assert!(matches!(
        verification,
        AccessTokenVerification::SignatureMismatch
    ));
}
