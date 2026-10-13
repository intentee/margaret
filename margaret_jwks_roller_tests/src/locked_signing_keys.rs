use margaret::framework::active_record::change::Change;
use margaret::framework::active_record::model::Model;
use margaret_database::transaction::Transaction;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_key_set_name::SIGNING_KEY_SET_NAME;

/// # Panics
///
/// Panics when the database cannot lock the stored signing keys.
pub async fn locked_signing_keys(
    transaction: &Transaction<'_>,
    generation: i64,
) -> Change<SigningKeySet> {
    SigningKeySet::query()
        .name
        .eq(SIGNING_KEY_SET_NAME.to_string())
        .update(transaction, |columns| columns.generation.to(generation))
        .await
        .expect("the database locks the signing keys")
}
