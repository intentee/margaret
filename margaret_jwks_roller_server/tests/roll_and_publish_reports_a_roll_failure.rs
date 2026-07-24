use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_roller_server::jwks_document_holder::JwksDocumentHolder;
use margaret_jwks_roller_server::jwks_roller_server_error::JwksRollerServerError;
use margaret_jwks_roller_server::roll_and_publish::roll_and_publish;
use margaret_jwks_roller_tests::unreachable_jwks_secret_storage::UnreachableJwksSecretStorage;

#[test]
fn roll_and_publish_reports_a_roll_failure() {
    let error = roll_and_publish(
        &UnreachableJwksSecretStorage,
        &JwksSecretHolder::default(),
        &JwksDocumentHolder::default(),
    )
    .expect_err("the backend cannot be reached");

    assert!(matches!(error, JwksRollerServerError::SecretRoll(_)));
}
