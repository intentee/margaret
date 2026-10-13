use serde_json::Value;
use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::ignored_key_reason::IgnoredKeyReason;
use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_id::KeyId;
use margaret_jws_verification::key_set_assembly::KeySetAssembly;
use margaret_jws_verification::public_jwk_material::public_jwk_material;
use margaret_jws_verification::verification_key::VerificationKey;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;

fn verifies(jwk: &Jwk, token: &str) -> bool {
    let material = public_jwk_material(jwk)
        .continue_value()
        .expect("the published jwk restores its material");
    let KeySetAssembly::Assembled(key_set) =
        VerificationKeySet::assemble(vec![VerificationKey::new(
            KeyId::new("kid".to_string()),
            material,
        )])
    else {
        panic!("one key assembles");
    };
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(token) else {
        panic!("the token parses");
    };

    matches!(key_set.verify(&jws), JwsVerification::Verified(_))
}

fn claims() -> Value {
    json!({ "sub": "subject" })
}

#[test]
fn restores_the_material_of_a_published_ec_jwk() {
    let key = FixtureKey::generate(Curve::P256, "kid");

    assert!(verifies(&key.jwk(), &key.token(&key.header(), &claims())));
}

#[test]
fn restores_the_material_of_a_published_rsa_jwk() {
    let key = FixtureRsaKey::load("kid");

    assert!(verifies(&key.jwk(), &key.token(&key.header(), &claims())));
}

#[test]
fn rejects_a_published_jwk_declaring_an_algorithm_of_another_key_type() {
    let mut ec_jwk = FixtureKey::generate(Curve::P256, "kid").ec_jwk();

    ec_jwk.alg = Some(JwsAlgorithm::Rs256);

    assert!(matches!(
        public_jwk_material(&Jwk::Ec(ec_jwk)).break_value(),
        Some(IgnoredKeyReason::Algorithm(_))
    ));
}

#[test]
fn rejects_a_published_jwk_whose_point_is_off_its_curve() {
    let mut ec_jwk = FixtureKey::generate(Curve::P256, "kid").ec_jwk();

    ec_jwk.x = ec_jwk.y.clone();

    assert!(matches!(
        public_jwk_material(&Jwk::Ec(ec_jwk)).break_value(),
        Some(IgnoredKeyReason::Material(_))
    ));
}
