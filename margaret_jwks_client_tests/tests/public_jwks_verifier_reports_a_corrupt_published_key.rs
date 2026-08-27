use std::sync::Arc;

use margaret_jwks_client::jwks_client_error::JwksClientError;
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
async fn public_jwks_verifier_reports_a_corrupt_published_key() {
    let mut keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "kid".to_string(),
    })
    .expect("the keypair generates");
    let token = keypair
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

    keypair.public.x = "invalid @@@".to_string();

    let holder = PublicJwksHolder::default();

    holder.set(Some(Arc::new(PublicJwks {
        keys: vec![JwkPublic::Ec(keypair.public)],
    })));

    let result =
        PublicJwksVerifier::new(holder).verify::<TestClaims>(&token, test_instant(1_700_000_000));

    assert!(matches!(result, Err(JwksClientError::TokenVerification(_))));
}
