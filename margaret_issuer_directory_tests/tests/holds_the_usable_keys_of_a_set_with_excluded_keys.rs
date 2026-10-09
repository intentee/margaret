use serde_json::json;

use margaret_issuer_directory_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_issuer_directory_tests::json_handler::json_handler;
use margaret_issuer_directory_tests::localhost_jwks_endpoint::localhost_jwks_endpoint;
use margaret_issuer_directory_tests::running_fixture_issuer::RunningFixtureIssuer;
use margaret_issuer_key_set::held_key_set::HeldKeySet;
use margaret_issuer_key_set::key_set_holding::KeySetHolding;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;

#[tokio::test(flavor = "multi_thread")]
async fn holds_the_usable_keys_of_a_set_with_excluded_keys() {
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: json_handler(404, &json!({})),
        key_set: json_handler(
            200,
            &json!({ "keys": [
                FixtureRsaKey::load("rsa-kid").jwk(),
                { "kty": "oct", "k": "c2VjcmV0", "kid": "disclosed" },
                { "kty": "AKP", "kid": "ignored" },
            ] }),
        ),
    })
    .await;

    let refresh = localhost_jwks_endpoint()
        .first_poll(issuer.request_client())
        .await;

    issuer.stop().await;

    let KeySetRefresh::Refreshed(KeySetHolding::Held(HeldKeySet { key_set, .. })) = refresh else {
        panic!("the usable keys of the set are held");
    };
    let key = FixtureRsaKey::load("rsa-kid");
    let token = key.token(&key.header(), &json!({}));
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&token) else {
        panic!("the fixture token is a compact jws");
    };

    assert!(matches!(key_set.verify(&jws), JwsVerification::Verified(_)));
}
