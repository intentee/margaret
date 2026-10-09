use uuid::Uuid;

use margaret_authorization_grants::authorization_code_record::AuthorizationCodeRecord;
use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_authorization_grants::redemption_decision::RedemptionDecision;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;
use margaret_database_tests::started_database::StartedDatabase;

#[tokio::test]
async fn reports_a_code_redeemed_in_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;

    assert!(matches!(
        AuthorizationCodeRecord::redeem(
            &started.database,
            contract_token(),
            Uuid::new_v4(),
            contract_instant(),
            RedemptionDecision::Consume,
        )
        .await,
        Err(AuthorizationGrantsError::SweepRefreshFamilies(_))
    ));
}
