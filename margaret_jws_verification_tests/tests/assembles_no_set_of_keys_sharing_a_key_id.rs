use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::duplicate_key_id::DuplicateKeyId;
use margaret_jws_verification::key_set_assembly::KeySetAssembly;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

#[test]
fn assembles_no_set_of_keys_sharing_a_key_id() {
    let assembly = VerificationKeySet::assemble(vec![
        FixtureKey::generate(Curve::P256, "same").verification_key(),
        FixtureKey::generate(Curve::P256, "same").verification_key(),
    ]);

    assert!(matches!(
        assembly,
        KeySetAssembly::DuplicateKeyId(DuplicateKeyId { kid }) if kid.as_str() == "same"
    ));
}
