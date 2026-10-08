use uuid::Uuid;

use margaret_authorization_grants::code_redemption::CodeRedemption;
use margaret_authorization_grants::issued_code::IssuedCode;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;

use crate::contract_instant::contract_instant;
use crate::contract_issued_code::contract_issued_code;
use crate::contract_token::contract_token;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn issuing_a_code_sweeps_expired_codes(store: &dyn StoresAuthorizationGrants) {
    let now = contract_instant();
    let expired = contract_token();

    store
        .issue_code(
            expired,
            IssuedCode {
                expires_at: now,
                ..contract_issued_code()
            },
            now,
        )
        .await
        .expect("the store issues the expiring code");
    store
        .issue_code(contract_token(), contract_issued_code(), now)
        .await
        .expect("the store issues another code");

    assert_eq!(
        store
            .redeem_code(expired, Uuid::new_v4())
            .await
            .expect("the store redeems codes"),
        CodeRedemption::Unknown
    );
}
