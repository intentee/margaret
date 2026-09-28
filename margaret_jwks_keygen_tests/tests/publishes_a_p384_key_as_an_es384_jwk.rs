use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jose_parameters::key_use::KeyUse;
use margaret_jwks_keygen_tests::fixture_pair::fixture_pair;
use margaret_jws_verification::ec_jwk::EcJwk;
use margaret_jws_verification::jwk::Jwk;

#[test]
fn publishes_a_p384_key_as_an_es384_jwk() {
    let pair = fixture_pair(Curve::P384, "p384");
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

    assert_eq!(alg, Some(JwsAlgorithm::Es384));
    assert_eq!(crv, Curve::P384);
    assert_eq!(kid.as_ref(), Some(pair.kid()));
    assert_eq!(key_use, Some(KeyUse::Signature));
}
