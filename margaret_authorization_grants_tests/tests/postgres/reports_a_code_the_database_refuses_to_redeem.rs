use uuid::Uuid;

use margaret_authorization_grants::authorization_code_record::AuthorizationCodeRecord;
use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_authorization_grants::redemption_decision::RedemptionDecision;
use margaret_authorization_grants_tests::contract_issued_code::contract_issued_code;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;
use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_a_code_the_database_refuses_to_redeem() {
    let started = started_with_authorization_grants().await;
    let code = contract_token();
    let now = contract_instant();

    AuthorizationCodeRecord::issue(&started.database, code, contract_issued_code(), now)
        .await
        .expect("the database issues the code");
    started
        .administration
        .revoke(
            TablePrivilege::Update,
            TableNamespace::Framework,
            "authorization_codes",
        )
        .await;

    assert!(matches!(
        AuthorizationCodeRecord::redeem(
            &started.database,
            code,
            Uuid::new_v4(),
            now,
            RedemptionDecision::Consume
        )
        .await,
        Err(AuthorizationGrantsError::RedeemCode(_))
    ));
}
