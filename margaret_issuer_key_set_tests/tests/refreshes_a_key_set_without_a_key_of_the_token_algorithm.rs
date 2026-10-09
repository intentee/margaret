use std::ops::ControlFlow;
use std::sync::Arc;

use margaret_issuer_key_set::issuer_fetch_spacing::ISSUER_FETCH_SPACING;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_key_set::issuer_verification::IssuerVerification;
use margaret_issuer_key_set_tests::access_token_expectation::access_token_expectation;
use margaret_issuer_key_set_tests::kid_less_access_token::kid_less_access_token;
use margaret_issuer_key_set_tests::refresh_observation::RefreshObservation;
use margaret_issuer_key_set_tests::verified_while_polling::verified_while_polling;
use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::key_set_assembly::KeySetAssembly;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;
use margaret_jwt_verification::attribute_serialized_jwt::attribute_serialized_jwt;

#[tokio::test(start_paused = true)]
async fn refreshes_a_key_set_without_a_key_of_the_token_algorithm() {
    let other_algorithm = FixtureKey::generate(Curve::P384, "other-algorithm");
    let introduced = FixtureKey::generate(Curve::P256, "introduced");
    let KeySetAssembly::Assembled(held_set) =
        VerificationKeySet::assemble(vec![other_algorithm.verification_key()])
    else {
        panic!("the held key set assembles");
    };
    let KeySetAssembly::Assembled(refreshed_set) = VerificationKeySet::assemble(vec![
        other_algorithm.verification_key(),
        introduced.verification_key(),
    ]) else {
        panic!("the refreshed key set assembles");
    };
    let issuer_key_set = IssuerKeySet::awaiting();

    issuer_key_set.start_fetch();
    issuer_key_set.hold(Arc::new(held_set));
    tokio::time::advance(ISSUER_FETCH_SPACING).await;

    let token = kid_less_access_token(&introduced);
    let ControlFlow::Continue(attributed) =
        attribute_serialized_jwt(&token, &access_token_expectation())
    else {
        panic!("the token is attributed to its issuer");
    };
    let polled =
        verified_while_polling(&issuer_key_set, &attributed, Arc::new(refreshed_set)).await;

    assert_eq!(polled.refresh, RefreshObservation::Served);
    assert!(matches!(
        polled.verification,
        IssuerVerification::Verified(verified) if verified.claims["sub"] == "subject"
    ));
}
