use uuid::Uuid;

use margaret_authorization_grants::authorization_code_record::AuthorizationCodeRecord;
use margaret_authorization_grants::code_redemption::CodeRedemption;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_token::contract_token;

#[tokio::test]
async fn code_redemption_finds_no_unknown_code() {
    let started = started_with_authorization_grants().await;
    let database = started.database.as_ref();

    assert_eq!(
        AuthorizationCodeRecord::redeem(database, contract_token(), Uuid::new_v4())
            .await
            .expect("the database redeems codes"),
        CodeRedemption::Unknown
    );
}
