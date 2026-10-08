use serde::Deserialize;
use serde_json::json;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_token_signer_tests::unix_time::unix_time;

#[derive(Debug, Deserialize, PartialEq)]
struct DemoClaims {
    name: String,
}

#[test]
fn signs_access_token_claims_that_verify_with_the_current_key() {
    let secret = fresh_secret(SigningCurve::P256);
    let current_kid = secret.current().kid().clone();
    let store = rolled_store(secret);
    let signed = store
        .sign_access_token(&json!({ "name": "demo" }), unix_time(500))
        .expect("the claims are signed");
    let JwtVerification::Verified(verified) =
        store.verify_access_token::<DemoClaims>(&signed.signed_claims, unix_time(500))
    else {
        panic!("the signed claims verify");
    };

    assert_eq!(verified.kid, Some(current_kid));
    assert_eq!(
        verified.claims,
        DemoClaims {
            name: "demo".to_string()
        }
    );
}
