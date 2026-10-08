use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;

use crate::stored_revision::StoredRevision;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn signing_keys_start_absent(store: &dyn StoresSigningKeys) {
    assert_eq!(StoredRevision::loaded(store).await, StoredRevision::Absent);
}
