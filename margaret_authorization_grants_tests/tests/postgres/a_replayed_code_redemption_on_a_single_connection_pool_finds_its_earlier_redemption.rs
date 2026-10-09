use std::num::NonZeroUsize;

use uuid::Uuid;

use margaret::framework::database::max_connections::MaxConnections;
use margaret_authorization_grants::authorization_code_record::AuthorizationCodeRecord;
use margaret_authorization_grants::code_redemption::CodeRedemption;
use margaret_authorization_grants::redemption_decision::RedemptionDecision;
use margaret_authorization_grants_tests::contract_issued_code::contract_issued_code;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;

#[tokio::test]
async fn a_replayed_code_redemption_on_a_single_connection_pool_finds_its_earlier_redemption() {
    let started = started_with_authorization_grants().await;
    let database = started
        .separate_pool_of(MaxConnections {
            connections: NonZeroUsize::MIN,
        })
        .await;
    let code = contract_token();
    let first_family = Uuid::new_v4();

    AuthorizationCodeRecord::issue(
        database.as_ref(),
        code,
        contract_issued_code(),
        contract_instant(),
    )
    .await
    .expect("the database issues the code");
    AuthorizationCodeRecord::redeem(
        database.as_ref(),
        code,
        first_family,
        contract_instant(),
        RedemptionDecision::Consume,
    )
    .await
    .expect("the database redeems the code");

    assert_eq!(
        AuthorizationCodeRecord::redeem(
            database.as_ref(),
            code,
            Uuid::new_v4(),
            contract_instant(),
            RedemptionDecision::Consume,
        )
        .await
        .expect("the database reports the earlier redemption"),
        CodeRedemption::AlreadyRedeemed {
            family: first_family
        }
    );
}
