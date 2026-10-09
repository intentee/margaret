use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jose_parameters::key_use::KeyUse;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_pair::fixture_pair;
use margaret_jws_verification::ec_jwk::EcJwk;
use margaret_jws_verification::jwk::Jwk;

#[test]
fn publishes_a_p256_key_as_an_es256_jwk() {
    let pair = fixture_pair(SigningCurve::P256, "p256");
    let Jwk::Ec(EcJwk {
        alg,
        crv,
        kid,
        key_use,
        ..
    }) = pair.public_jwk().clone()
    else {
        panic!("the pair publishes an ec key");
    };

    assert_eq!(alg, Some(JwsAlgorithm::Es256));
    assert_eq!(crv, SigningCurve::P256.curve());
    assert_eq!(kid.as_ref(), Some(pair.kid()));
    assert_eq!(key_use, Some(KeyUse::Signature));
}
