use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::key_set_rejection::KeySetRejection;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

#[test]
fn rejects_a_set_with_a_duplicate_key_id() {
    let parsing = VerificationKeySet::from_jwks(vec![
        FixtureKey::generate(Curve::P256, "same").jwk(),
        FixtureKey::generate(Curve::P256, "same").jwk(),
    ]);

    assert!(matches!(
        parsing,
        KeySetParsing::Rejected(KeySetRejection::DuplicateKeyId { kid }) if kid.as_str() == "same"
    ));
}
