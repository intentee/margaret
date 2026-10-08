use uuid::Uuid;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_authorization_grants::refresh_family_record::RefreshFamilyRecord;
use margaret_authorization_grants_tests::contract_refresh_family::contract_refresh_family;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;
use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn opening_a_family_whose_first_token_is_refused_leaves_no_family() {
    let started = started_with_authorization_grants().await;
    let family = Uuid::new_v4();

    started
        .administration
        .revoke(
            TablePrivilege::Insert,
            TableNamespace::Framework,
            "refresh_tokens",
        )
        .await;

    assert!(matches!(
        RefreshFamilyRecord::open(
            &started.database,
            family,
            contract_refresh_family(),
            contract_token(),
            contract_instant(),
        )
        .await,
        Err(AuthorizationGrantsError::IssueFirstRefreshToken(_))
    ));
    assert!(matches!(
        RefreshFamilyRecord::query()
            .id
            .eq(family)
            .find(started.database.as_ref())
            .await
            .expect("the database finds refresh families"),
        Lookup::Missing
    ));
}
