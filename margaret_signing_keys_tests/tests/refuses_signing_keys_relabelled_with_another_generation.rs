use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_signing_keys::signing_keys_error::SigningKeysError;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys_tests::contract_revision::contract_revision;
use margaret_signing_keys_tests::fixture_signing_keys_encryption_key::fixture_signing_keys_encryption_key;

#[test]
fn refuses_signing_keys_relabelled_with_another_generation() {
    let SigningKeysRevision { sealed, .. } = contract_revision(0);

    assert!(matches!(
        SigningKeysRevision {
            generation: SigningKeysGeneration::new(2),
            sealed,
        }
        .opened(&fixture_signing_keys_encryption_key()),
        Err(SigningKeysError::DocumentOpen(_))
    ));
}
