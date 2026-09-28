use std::sync::Arc;

use serde_json::Value;
use serde_json::json;

use margaret_http::request_authorization::RequestAuthorization;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;
use margaret_oidc_client::oidc_token_rejection::OidcTokenRejection;
use margaret_oidc_client::oidc_token_verification::OidcTokenVerification;
use margaret_oidc_client::oidc_token_verifier::OidcTokenVerifier;
use margaret_oidc_client::presented_bearer::PresentedBearer;
use margaret_oidc_client_tests::localhost_trust::localhost_trust;
use margaret_oidc_client_tests::signed_id_token::SignedIdToken;

#[test]
fn oidc_token_verifier_rejects_a_token_for_another_audience() {
    let key = FixtureRsaKey::load("rsa-kid");
    let KeySetParsing::Accepted(key_set) = VerificationKeySet::from_jwks(vec![key.jwk()]) else {
        panic!("the fixture key set is accepted");
    };
    let verification_key_set_holder = VerificationKeySetHolder::default();

    verification_key_set_holder.set(Some(Arc::new(key_set)));

    let trust = localhost_trust();
    let verifier = OidcTokenVerifier::new(Arc::new(trust.clone()), verification_key_set_holder);
    let token = SignedIdToken {
        audience: json!(["margaret", "someone-else"]),
        exp: 9_999_999_999,
        typ: "JWT",
    }
    .signed_by(&key, &trust);

    let authorization = RequestAuthorization::parse(Some(&format!("Bearer {token}")));
    let presented =
        PresentedBearer::read(&authorization).expect("the system clock reads as a numeric date");

    assert!(matches!(
        verifier.verify::<Value>(&presented),
        OidcTokenVerification::Rejected(OidcTokenRejection::Token(JwtRejection::Claims(
            ClaimsRejection::AudienceMismatch { .. }
        )))
    ));
}
