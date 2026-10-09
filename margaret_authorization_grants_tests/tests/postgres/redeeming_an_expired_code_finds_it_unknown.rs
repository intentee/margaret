use uuid::Uuid;

use margaret_authorization_grants::authorization_code_record::AuthorizationCodeRecord;
use margaret_authorization_grants::code_redemption::CodeRedemption;
use margaret_authorization_grants::issued_code::IssuedCode;
use margaret_authorization_grants::redemption_decision::RedemptionDecision;
use margaret_authorization_grants_tests::contract_issued_code::contract_issued_code;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;

#[tokio::test]
async fn redeeming_an_expired_code_finds_it_unknown() {
    let started = started_with_authorization_grants().await;
    let database = started.database.as_ref();
    let now = contract_instant();
    let expired = contract_token();

    AuthorizationCodeRecord::issue(
        database,
        expired,
        IssuedCode {
            expires_at: now,
            ..contract_issued_code()
        },
        now,
    )
    .await
    .expect("the database issues the expiring code");

    assert_eq!(
        AuthorizationCodeRecord::redeem(
            database,
            expired,
            Uuid::new_v4(),
            now,
            RedemptionDecision::Consume,
        )
        .await
        .expect("the database redeems codes"),
        CodeRedemption::Unknown
    );
}
