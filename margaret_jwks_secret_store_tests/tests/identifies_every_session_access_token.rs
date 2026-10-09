use uuid::Uuid;

use margaret_identity_session::session_access_token_claims::SessionAccessTokenClaims;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_jwks_secret_store_tests::unix_time::unix_time;
use margaret_jwt_verification::jwt_verification::JwtVerification;

fn signed_token_identifier(store: &JwksSecretStore) -> String {
    let signed = store.issue_session_access_token(
        &SessionAccessTokenClaims {
            auth_time: unix_time(400),
            sid: Uuid::from_u128(7),
            sub: Uuid::from_u128(9),
        },
        "browser",
        unix_time(500),
    );
    let JwtVerification::Verified(verified) =
        store.verify_session_access_token(&signed.signed_claims, "browser", unix_time(500))
    else {
        panic!("the session access token verifies");
    };

    verified
        .registered
        .jti
        .expect("the session access token carries a token identifier")
}

#[test]
fn identifies_every_session_access_token() {
    let store = rolled_store(fresh_secret(SigningCurve::P256));

    assert_ne!(
        signed_token_identifier(&store),
        signed_token_identifier(&store)
    );
}
