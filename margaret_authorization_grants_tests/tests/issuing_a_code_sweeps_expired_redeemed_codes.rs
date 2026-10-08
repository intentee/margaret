use uuid::Uuid;

use margaret_authorization_grants::authorization_code_record::AuthorizationCodeRecord;
use margaret_authorization_grants::code_redemption::CodeRedemption;
use margaret_authorization_grants::issued_code::IssuedCode;
use margaret_authorization_grants_tests::contract_issued_code::contract_issued_code;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;

#[tokio::test]
async fn issuing_a_code_sweeps_expired_redeemed_codes() {
    let started = started_with_authorization_grants().await;
    let database = started.database.as_ref();
    let now = contract_instant();
    let redeemed = contract_token();

    AuthorizationCodeRecord::issue(
        database,
        redeemed,
        IssuedCode {
            expires_at: now,
            ..contract_issued_code()
        },
        now,
    )
    .await
    .expect("the database issues the expiring code");
    AuthorizationCodeRecord::redeem(database, redeemed, Uuid::new_v4())
        .await
        .expect("the database redeems the expiring code");
    AuthorizationCodeRecord::issue(database, contract_token(), contract_issued_code(), now)
        .await
        .expect("the database issues another code");

    assert_eq!(
        AuthorizationCodeRecord::redeem(database, redeemed, Uuid::new_v4())
            .await
            .expect("the database redeems codes"),
        CodeRedemption::Unknown
    );
}
