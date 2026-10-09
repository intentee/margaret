use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_roller::signing_keys_creation::SigningKeysCreation;
use margaret_jwks_roller::signing_keys_replacement::SigningKeysReplacement;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;

use crate::contract_revision::contract_revision;
use crate::stored_revision::StoredRevision;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn signing_keys_replacement_refuses_another_generation(store: &dyn StoresSigningKeys) {
    let replacement = contract_revision(1);

    assert_eq!(
        store
            .create_signing_keys(&contract_revision(0))
            .await
            .expect("the store creates signing keys"),
        SigningKeysCreation::Created
    );
    assert_eq!(
        store
            .replace_signing_keys(SigningKeysGeneration::FIRST, &replacement)
            .await
            .expect("the store replaces signing keys"),
        SigningKeysReplacement::Replaced
    );
    assert_eq!(
        store
            .replace_signing_keys(SigningKeysGeneration::FIRST, &contract_revision(1))
            .await
            .expect("the store refuses a replacement of another generation"),
        SigningKeysReplacement::Superseded
    );
    assert_eq!(
        StoredRevision::loaded(store).await,
        StoredRevision::of(&replacement)
    );
}
