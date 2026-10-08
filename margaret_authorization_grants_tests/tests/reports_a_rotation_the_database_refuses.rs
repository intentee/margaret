use uuid::Uuid;

use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_authorization_grants::refresh_family_record::RefreshFamilyRecord;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_authorization_grants_tests::contract_refresh_family::contract_refresh_family;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;
use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_a_rotation_the_database_refuses() {
    let started = started_with_authorization_grants().await;
    let presented = contract_token();
    let now = contract_instant();

    RefreshFamilyRecord::open(
        &started.database,
        Uuid::new_v4(),
        contract_refresh_family(),
        presented,
        now,
    )
    .await
    .expect("the database opens the refresh family");

    started
        .administration
        .revoke(
            TablePrivilege::Update,
            TableNamespace::Framework,
            "refresh_tokens",
        )
        .await;

    assert!(matches!(
        RefreshTokenRecord::rotate(&started.database, presented, contract_token(), now).await,
        Err(AuthorizationGrantsError::RotateRefreshToken(_))
    ));
}
