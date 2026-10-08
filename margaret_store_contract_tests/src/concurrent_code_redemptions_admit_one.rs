use futures_util::future::join_all;
use uuid::Uuid;

use margaret_authorization_grants::code_redemption::CodeRedemption;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;

use crate::contract_instant::contract_instant;
use crate::contract_issued_code::contract_issued_code;
use crate::contract_token::contract_token;
use crate::racing_instances::RACING_INSTANCES;

struct RacedRedemption {
    family: Uuid,
    redemption: CodeRedemption,
}

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn concurrent_code_redemptions_admit_one(store: &dyn StoresAuthorizationGrants) {
    let code = contract_token();
    let issued = contract_issued_code();

    store
        .issue_code(code, issued.clone(), contract_instant())
        .await
        .expect("the store issues the code");

    let raced: Vec<RacedRedemption> = join_all((0..RACING_INSTANCES).map(|_| async move {
        let family = Uuid::new_v4();

        RacedRedemption {
            family,
            redemption: store
                .redeem_code(code, family)
                .await
                .expect("the store redeems the code"),
        }
    }))
    .await;
    let winners: Vec<&RacedRedemption> = raced
        .iter()
        .filter(|raced| raced.redemption == CodeRedemption::Redeemed(Box::new(issued.clone())))
        .collect();

    assert_eq!(winners.len(), 1);
    assert_eq!(
        raced
            .iter()
            .filter(|raced| raced.redemption
                == CodeRedemption::AlreadyRedeemed {
                    family: winners[0].family
                })
            .count(),
        RACING_INSTANCES - 1
    );
}
