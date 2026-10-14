use margaret::framework::signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret::framework::signing_keys::stored_signing_keys::StoredSigningKeys;

/// # Panics
///
/// Panics when no signing keys are stored.
#[must_use]
pub fn stored_signing_keys_revision(stored: StoredSigningKeys) -> SigningKeysRevision {
    let StoredSigningKeys::Stored(revision) = stored else {
        panic!("the cluster stores its signing keys");
    };

    revision
}

#[cfg(test)]
mod tests {
    use margaret::framework::signing_keys::stored_signing_keys::StoredSigningKeys;

    use super::stored_signing_keys_revision;

    #[test]
    #[should_panic(expected = "the cluster stores its signing keys")]
    fn refuses_signing_keys_the_cluster_never_stored() {
        let _ = stored_signing_keys_revision(StoredSigningKeys::Absent);
    }
}
