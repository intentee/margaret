use base64ct::Base64;
use base64ct::Encoding;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_signing_keys::constants::SIGNING_KEY_SET_NAME;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys_tests::fixture_signing_keys_encryption_key::fixture_signing_keys_encryption_key;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

#[tokio::test]
async fn stores_the_private_keys_only_sealed() {
    let started = started_with_signing_keys().await;
    let database = started.database.as_ref();
    let secret = fresh_secret(SigningCurve::P256);

    SigningKeySet::create(
        database,
        &SigningKeysRevision::sealed(&secret, &fixture_signing_keys_encryption_key())
            .expect("the signing keys are sealed"),
    )
    .await
    .expect("the signing keys are created");

    let Lookup::Found(SigningKeySet {
        sealed_document, ..
    }) = SigningKeySet::query()
        .name
        .eq(SIGNING_KEY_SET_NAME.to_string())
        .find(database)
        .await
        .expect("the stored row loads")
    else {
        panic!("the signing keys are stored");
    };

    assert!(
        [
            secret.current().signing_key().pkcs8(),
            secret.next().signing_key().pkcs8(),
            secret.rsa().current().signing_key().pkcs8(),
            secret.rsa().next().signing_key().pkcs8(),
        ]
        .into_iter()
        .map(|pkcs8| Base64::encode_string(pkcs8))
        .all(|encoded| {
            !sealed_document
                .windows(encoded.len())
                .any(|window| window == encoded.as_bytes())
        })
    );
}
