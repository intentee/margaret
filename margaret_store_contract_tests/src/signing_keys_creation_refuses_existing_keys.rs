use margaret_jwks_roller::signing_keys_creation::SigningKeysCreation;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;

use crate::contract_revision::contract_revision;
use crate::stored_revision::StoredRevision;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn signing_keys_creation_refuses_existing_keys(store: &dyn StoresSigningKeys) {
    let first = contract_revision(0);

    assert_eq!(
        store
            .create_signing_keys(&first)
            .await
            .expect("the store creates signing keys"),
        SigningKeysCreation::Created
    );
    assert_eq!(
        store
            .create_signing_keys(&contract_revision(0))
            .await
            .expect("the store refuses to create signing keys twice"),
        SigningKeysCreation::AlreadyCreated
    );
    assert_eq!(
        StoredRevision::loaded(store).await,
        StoredRevision::of(&first)
    );
}
