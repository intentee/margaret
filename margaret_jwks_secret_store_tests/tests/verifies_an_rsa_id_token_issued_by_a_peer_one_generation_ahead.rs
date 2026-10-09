use serde_json::Value;
use uuid::Uuid;

use margaret_identity_session::id_token_claims::IdTokenClaims;
use margaret_jwks_secret_store::id_token_signing::IdTokenSigning;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwks_secret_store_tests::peer_rollers::PeerRollers;
use margaret_jwt_verification::attribute_serialized_jwt::attribute_serialized_jwt;
use margaret_jwt_verification::expected_audience::ExpectedAudience;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn verifies_an_rsa_id_token_issued_by_a_peer_one_generation_ahead() {
    let peers = PeerRollers::one_generation_apart().await;
    let id_token = JwksSecretStore::create(peers.rolled, fixture_issuance())
        .issue_id_token(
            &IdTokenClaims {
                auth_time: NumericDate::from(unix_time(500)),
                client_id: "portal",
                nonce: None,
                subject: Uuid::nil(),
            },
            IdTokenSigning::Rsa,
            unix_time(500),
        )
        .expect("the rsa key signs the id token");
    let attributed = attribute_serialized_jwt(
        &id_token,
        &JwtExpectation {
            audience: ExpectedAudience::Sole("portal"),
            issuer: fixture_issuance().issuer,
        },
    )
    .continue_value()
    .expect("the id token is addressed to the portal");

    assert!(matches!(
        JwksSecretStore::create(peers.lagging, fixture_issuance())
            .verify_issued_jwt::<Value, IdTokenProfile>(
                &attributed,
                NumericDate::from(unix_time(500))
            ),
        JwtVerification::Verified(_)
    ));
}
