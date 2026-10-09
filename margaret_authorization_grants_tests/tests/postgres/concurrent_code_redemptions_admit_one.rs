use futures_util::future::join_all;
use uuid::Uuid;

use margaret_authorization_grants::authorization_code_record::AuthorizationCodeRecord;
use margaret_authorization_grants::authorization_grant::AuthorizationGrant;
use margaret_authorization_grants::code_redemption::CodeRedemption;
use margaret_authorization_grants::redemption_decision::RedemptionDecision;
use margaret_authorization_grants_tests::contract_issued_code::contract_issued_code;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;
use margaret_database_tests::racing_instances::RACING_INSTANCES;

struct RacedRedemption {
    family: Uuid,
    redemption: CodeRedemption<AuthorizationGrant>,
}

#[tokio::test]
async fn concurrent_code_redemptions_admit_one() {
    let started = started_with_authorization_grants().await;
    let database = started.database.as_ref();
    let code = contract_token();
    let issued = contract_issued_code();

    AuthorizationCodeRecord::issue(database, code, issued.clone(), contract_instant())
        .await
        .expect("the database issues the code");

    let raced: Vec<RacedRedemption> = join_all((0..RACING_INSTANCES).map(|_| async move {
        let family = Uuid::new_v4();

        RacedRedemption {
            family,
            redemption: AuthorizationCodeRecord::redeem(
                database,
                code,
                family,
                contract_instant(),
                RedemptionDecision::Consume,
            )
            .await
            .expect("the database redeems the code"),
        }
    }))
    .await;
    let winners: Vec<&RacedRedemption> = raced
        .iter()
        .filter(|raced| raced.redemption == CodeRedemption::Redeemed(issued.grant.clone()))
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
