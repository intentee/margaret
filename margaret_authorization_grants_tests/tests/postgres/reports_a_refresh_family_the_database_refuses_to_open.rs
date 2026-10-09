use uuid::Uuid;

use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_authorization_grants_tests::contract_refresh_family::contract_refresh_family;
use margaret_authorization_grants_tests::refresh_family_redemption::refresh_family_redemption;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;
use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_a_refresh_family_the_database_refuses_to_open() {
    let started = started_with_authorization_grants().await;

    started
        .administration
        .revoke(
            TablePrivilege::Insert,
            TableNamespace::Framework,
            "refresh_families",
        )
        .await;

    assert!(matches!(
        refresh_family_redemption(
            &started.database,
            Uuid::new_v4(),
            contract_refresh_family(),
            contract_token(),
            contract_instant(),
        )
        .await,
        Err(AuthorizationGrantsError::OpenRefreshFamily(_))
    ));
}
