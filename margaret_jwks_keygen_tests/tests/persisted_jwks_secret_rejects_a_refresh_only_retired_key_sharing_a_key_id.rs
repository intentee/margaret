use std::iter;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::persisted_document::persisted_document;
use margaret_jwks_keygen_tests::restored_document::restored_document;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller::signing_key_retention::signing_key_retention;

#[test]
fn persisted_jwks_secret_rejects_a_refresh_only_retired_key_sharing_a_key_id() {
    let rolls_beyond_token_retention = usize::try_from(
        signing_key_retention()
            .token
            .as_secs()
            .div_ceil(JWKS_ROLL_INTERVAL.as_secs())
            + 1,
    )
    .expect("the roll count fits");
    let secret = iter::successors(Some(fresh_secret(SigningCurve::P256)), |secret| {
        Some(rolled_secret(secret))
    })
    .nth(rolls_beyond_token_retention)
    .expect("the secret rolls");
    let oldest = secret.retired().len() - 1;
    let mut document = persisted_document(&secret);

    assert!(
        secret.retired()[oldest]
            .retired_at()
            .after(signing_key_retention().token)
            <= secret.rolled_at()
    );

    document["ec"]["retired"][oldest]["public"]["kid"] = document["ec"]["current"]["kid"].clone();

    assert!(matches!(
        restored_document(document),
        Err(JwksKeyError::DuplicateKeyId { .. })
    ));
}
