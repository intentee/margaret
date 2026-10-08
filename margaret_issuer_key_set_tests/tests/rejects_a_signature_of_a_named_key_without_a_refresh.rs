use std::ops::ControlFlow;
use std::sync::Arc;

use serde_json::json;

use margaret_issuer_key_set::issuer_fetch_spacing::ISSUER_FETCH_SPACING;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_key_set::issuer_verification::IssuerVerification;
use margaret_issuer_key_set_tests::access_token_claims::access_token_claims;
use margaret_issuer_key_set_tests::access_token_expectation::access_token_expectation;
use margaret_issuer_key_set_tests::refresh_observation::RefreshObservation;
use margaret_issuer_key_set_tests::verified_while_polling::verified_while_polling;
use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jws_verification::key_selection::KeySelection;
use margaret_jws_verification::key_set_assembly::KeySetAssembly;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;
use margaret_jwt_verification::attribute_serialized_jwt::attribute_serialized_jwt;
use margaret_jwt_verification::jwt_rejection::JwtRejection;

#[tokio::test(start_paused = true)]
async fn rejects_a_signature_of_a_named_key_without_a_refresh() {
    let published = FixtureKey::generate(Curve::P256, "published");
    let impostor = FixtureKey::generate(Curve::P256, "published");
    let KeySetAssembly::Assembled(held_set) =
        VerificationKeySet::assemble(vec![published.verification_key()])
    else {
        panic!("the held key set assembles");
    };
    let KeySetAssembly::Assembled(unserved_set) =
        VerificationKeySet::assemble(vec![impostor.verification_key()])
    else {
        panic!("the unserved key set assembles");
    };
    let issuer_key_set = IssuerKeySet::awaiting();

    issuer_key_set.start_fetch();
    issuer_key_set.hold(Arc::new(held_set));
    tokio::time::advance(ISSUER_FETCH_SPACING).await;

    let mut header = impostor.header();

    header["typ"] = json!(JwtType::AccessToken.wire_name());

    let token = impostor.token(&header, &access_token_claims());
    let ControlFlow::Continue(attributed) =
        attribute_serialized_jwt(&token, &access_token_expectation())
    else {
        panic!("the token is attributed to its issuer");
    };
    let polled = verified_while_polling(&issuer_key_set, &attributed, Arc::new(unserved_set)).await;

    assert_eq!(polled.refresh, RefreshObservation::NotRequested);
    assert!(matches!(
        polled.verification,
        IssuerVerification::Rejected(JwtRejection::Jws(JwsRejection::SignatureMismatch {
            selection: KeySelection::ByKeyId,
            ..
        }))
    ));
}
