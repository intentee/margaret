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
async fn refreshes_the_key_set_after_the_sole_key_of_the_algorithm_rotates() {
    let retired = FixtureKey::generate(Curve::P256, "retired");
    let rotated = FixtureKey::generate(Curve::P256, "rotated");
    let KeySetAssembly::Assembled(retired_set) =
        VerificationKeySet::assemble(vec![retired.verification_key()])
    else {
        panic!("the retired key set assembles");
    };
    let KeySetAssembly::Assembled(rotated_set) =
        VerificationKeySet::assemble(vec![rotated.verification_key()])
    else {
        panic!("the rotated key set assembles");
    };
    let issuer_key_set = IssuerKeySet::awaiting();

    issuer_key_set.start_fetch();
    issuer_key_set.hold(Arc::new(retired_set));
    tokio::time::advance(ISSUER_FETCH_SPACING).await;

    let token = kid_less_access_token(&rotated);
    let ControlFlow::Continue(attributed) =
        attribute_serialized_jwt(&token, &access_token_expectation())
    else {
        panic!("the token is attributed to its issuer");
    };
    let polled = verified_while_polling(&issuer_key_set, &attributed, Arc::new(rotated_set)).await;

    assert_eq!(polled.refresh, RefreshObservation::Served);
    assert!(matches!(
        polled.verification,
        IssuerVerification::Verified(verified) if verified.claims["sub"] == "subject"
    ));
}
