use std::sync::Arc;

use serde_json::Value;

use margaret_http::bearer_challenge::BearerChallenge;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;
use margaret_oidc_client::oidc_token_rejection::OidcTokenRejection;
use margaret_oidc_client::oidc_token_verification::OidcTokenVerification;
use margaret_oidc_client::oidc_token_verifier::OidcTokenVerifier;
use margaret_oidc_client::presented_bearer::PresentedBearer;
use margaret_oidc_client_tests::localhost_trust::localhost_trust;

#[test]
fn oidc_token_verifier_challenges_a_token_that_is_not_a_jws_as_an_invalid_token() {
    let KeySetParsing::Accepted(key_set) =
        VerificationKeySet::from_jwks(vec![FixtureRsaKey::load("rsa-kid").jwk()])
    else {
        panic!("the fixture key set is accepted");
    };
    let verification_key_set_holder = VerificationKeySetHolder::default();

    verification_key_set_holder.set(Some(Arc::new(key_set)));

    let verifier = OidcTokenVerifier::new(Arc::new(localhost_trust()), verification_key_set_holder);
    let authorization = RequestAuthorization::parse(Some("Bearer opaque-token"));
    let presented =
        PresentedBearer::read(&authorization).expect("the system clock reads as a numeric date");

    let OidcTokenVerification::Rejected(rejection) = verifier.verify::<Value>(&presented) else {
        panic!("a token that is not a jws is rejected");
    };

    assert!(matches!(
        &rejection,
        OidcTokenRejection::UnparseableToken(unparseable)
            if matches!(unparseable.as_ref(), JwsRejection::NotCompactJws)
    ));
    assert_eq!(rejection.challenge(), BearerChallenge::InvalidToken);
}
