use uuid::Uuid;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_authorization_grants::pending_authorization_record::PendingAuthorizationRecord;
use margaret_authorization_grants_tests::contract_pending_authorization::contract_pending_authorization;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_sql_identifier::qualified_table::qualified_table;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_a_malformed_stored_grant() {
    let started = started_with_authorization_grants().await;
    let id = Uuid::new_v4();

    PendingAuthorizationRecord::hold(
        &started.database,
        id,
        contract_pending_authorization(),
        contract_instant(),
    )
    .await
    .expect("the database holds the pending authorization");
    started
        .administration
        .execute(&format!(
            "UPDATE {} SET \"grant\" = 'not json'",
            qualified_table(TableNamespace::Framework, "pending_authorizations")
        ))
        .await;

    assert!(matches!(
        PendingAuthorizationRecord::take(&started.database, id).await,
        Err(AuthorizationGrantsError::TakePendingAuthorization(
            ActiveRecordError::MalformedJson { .. }
        ))
    ));
}
