use uuid::Uuid;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_authorization_grants_tests::contract_refresh_family::contract_refresh_family;
use margaret_authorization_grants_tests::opened_refresh_family::opened_refresh_family;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;
use margaret_sql_identifier::qualified_table::qualified_table;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_malformed_stored_scopes() {
    let started = started_with_authorization_grants().await;
    let first_token = contract_token();

    opened_refresh_family(
        &started.database,
        Uuid::new_v4(),
        contract_refresh_family(),
        first_token,
        contract_instant(),
    )
    .await;
    started
        .administration
        .execute(&format!(
            "UPDATE {} SET scopes = 'not json'",
            qualified_table(TableNamespace::Framework, "refresh_families")
        ))
        .await;

    assert!(matches!(
        RefreshTokenRecord::lookup(&started.database, first_token).await,
        Err(AuthorizationGrantsError::FindRefreshToken(
            ActiveRecordError::MalformedJson { .. }
        ))
    ));
}
