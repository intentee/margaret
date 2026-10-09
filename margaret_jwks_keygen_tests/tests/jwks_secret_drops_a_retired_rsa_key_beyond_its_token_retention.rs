use std::iter;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller::signing_key_retention::signing_key_retention;
use margaret_jws_verification::jwk::Jwk;

#[test]
fn jwks_secret_drops_a_retired_rsa_key_beyond_its_token_retention() {
    let rolls_beyond_retention = usize::try_from(
        signing_key_retention()
            .token
            .as_secs()
            .div_ceil(JWKS_ROLL_INTERVAL.as_secs())
            + 1,
    )
    .expect("the roll count fits");
    let generations: Vec<JwksSecret> =
        iter::successors(Some(fresh_secret(SigningCurve::P256)), |secret| {
            Some(rolled_secret(secret))
        })
        .take(rolls_beyond_retention + 1)
        .collect();
    let lapsed = &generations[rolls_beyond_retention];

    assert_eq!(
        lapsed
            .rsa()
            .retired()
            .iter()
            .map(|retired| retired.public_jwk().clone())
            .collect::<Vec<Jwk>>(),
        generations[1..rolls_beyond_retention]
            .iter()
            .rev()
            .map(|generation| generation.rsa().current().public_jwk().clone())
            .collect::<Vec<Jwk>>()
    );
}
