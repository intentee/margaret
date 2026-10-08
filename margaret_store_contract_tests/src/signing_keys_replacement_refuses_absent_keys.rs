use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_roller::signing_keys_replacement::SigningKeysReplacement;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;

use crate::contract_revision::contract_revision;
use crate::stored_revision::StoredRevision;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn signing_keys_replacement_refuses_absent_keys(store: &dyn StoresSigningKeys) {
    assert_eq!(
        store
            .replace_signing_keys(SigningKeysGeneration::FIRST, &contract_revision(1))
            .await
            .expect("the store refuses to replace absent signing keys"),
        SigningKeysReplacement::Superseded
    );
    assert_eq!(StoredRevision::loaded(store).await, StoredRevision::Absent);
}
