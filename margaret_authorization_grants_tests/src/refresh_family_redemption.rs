use uuid::Uuid;

use margaret::framework::database::database::Database;
use margaret_authorization_grants::authorization_code_lifetime::AUTHORIZATION_CODE_LIFETIME;
use margaret_authorization_grants::authorization_code_record::AuthorizationCodeRecord;
use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_authorization_grants::code_redemption::CodeRedemption;
use margaret_authorization_grants::family_opening::FamilyOpening;
use margaret_authorization_grants::issued_code::IssuedCode;
use margaret_authorization_grants::redemption_decision::RedemptionDecision;
use margaret_authorization_grants::refresh_family::RefreshFamily;
use margaret_database_tests::contract_token::contract_token;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

use crate::contract_grant::contract_grant;

/// # Errors
///
/// Returns `AuthorizationGrantsError` when the code cannot be redeemed with the family.
///
/// # Panics
///
/// Panics when the database cannot issue the code that opens the family.
pub async fn refresh_family_redemption(
    database: &Database,
    family: Uuid,
    refresh_family: RefreshFamily,
    first_token: TokenDigest,
    now: NumericDate,
) -> Result<CodeRedemption<()>, AuthorizationGrantsError> {
    let code = contract_token();

    AuthorizationCodeRecord::issue(
        database,
        code,
        IssuedCode {
            expires_at: now.after(AUTHORIZATION_CODE_LIFETIME),
            grant: contract_grant(),
        },
        now,
    )
    .await
    .expect("the database issues the code that opens the family");

    AuthorizationCodeRecord::redeem(database, code, family, now, |_grant| {
        RedemptionDecision::OpenFamily {
            decided: (),
            opening: FamilyOpening {
                family: refresh_family,
                first_token,
            },
        }
    })
    .await
}
