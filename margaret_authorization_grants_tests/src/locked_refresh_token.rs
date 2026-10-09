use margaret::framework::active_record::change::Change;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::transaction::Transaction;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_token_digest::token_digest::TokenDigest;

/// # Panics
///
/// Panics when the database cannot lock the current refresh token.
pub async fn locked_refresh_token(
    transaction: &Transaction<'_>,
    token: TokenDigest,
) -> Change<RefreshTokenRecord> {
    RefreshTokenRecord::query()
        .token
        .eq(token.as_bytes().to_vec())
        .when(|stored| stored.current.eq(true))
        .update(transaction, |columns| columns.current.to(true))
        .await
        .expect("the database locks the current refresh token")
}
