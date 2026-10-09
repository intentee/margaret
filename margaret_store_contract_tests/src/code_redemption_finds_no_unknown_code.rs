use uuid::Uuid;

use margaret_authorization_grants::code_redemption::CodeRedemption;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;

use crate::contract_token::contract_token;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn code_redemption_finds_no_unknown_code(store: &dyn StoresAuthorizationGrants) {
    assert_eq!(
        store
            .redeem_code(contract_token(), Uuid::new_v4())
            .await
            .expect("the store redeems codes"),
        CodeRedemption::Unknown
    );
}
