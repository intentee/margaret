use margaret_authorization_grants::authorization_code_record::AuthorizationCodeRecord;
use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_authorization_grants_tests::contract_issued_code::contract_issued_code;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;
use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_a_code_the_database_refuses_to_issue() {
    let started = started_with_authorization_grants().await;

    started
        .administration
        .revoke(
            TablePrivilege::Insert,
            TableNamespace::Framework,
            "authorization_codes",
        )
        .await;

    assert!(matches!(
        AuthorizationCodeRecord::issue(
            &started.database,
            contract_token(),
            contract_issued_code(),
            contract_instant(),
        )
        .await,
        Err(AuthorizationGrantsError::IssueCode(_))
    ));
}
