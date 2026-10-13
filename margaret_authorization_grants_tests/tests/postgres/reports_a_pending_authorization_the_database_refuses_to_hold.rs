use uuid::Uuid;

use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_authorization_grants::pending_authorization_record::PendingAuthorizationRecord;
use margaret_authorization_grants_tests::contract_pending_authorization::contract_pending_authorization;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_a_pending_authorization_the_database_refuses_to_hold() {
    let started = started_with_authorization_grants().await;

    started
        .administration
        .revoke(
            TablePrivilege::Insert,
            TableNamespace::Framework,
            "pending_authorizations",
        )
        .await;

    assert!(matches!(
        PendingAuthorizationRecord::hold(
            &started.database,
            Uuid::new_v4(),
            contract_pending_authorization(),
            contract_instant(),
        )
        .await,
        Err(AuthorizationGrantsError::HoldPendingAuthorization(_))
    ));
}
