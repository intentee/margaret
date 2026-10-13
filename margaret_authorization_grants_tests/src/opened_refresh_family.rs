use uuid::Uuid;

use margaret::framework::database::database::Database;
use margaret_authorization_grants::code_redemption::CodeRedemption;
use margaret_authorization_grants::refresh_family::RefreshFamily;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

use crate::refresh_family_redemption::refresh_family_redemption;

/// # Panics
///
/// Panics when the database cannot open the family through the redemption of a code.
pub async fn opened_refresh_family(
    database: &Database,
    family: Uuid,
    refresh_family: RefreshFamily,
    first_token: TokenDigest,
    now: NumericDate,
) {
    assert_eq!(
        refresh_family_redemption(database, family, refresh_family, first_token, now)
            .await
            .expect("the database opens the refresh family"),
        CodeRedemption::Redeemed(())
    );
}
