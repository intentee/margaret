use std::sync::Arc;

use uuid::Uuid;

use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;
use margaret_authorization_grants_database::authorization_grants_database_error::AuthorizationGrantsDatabaseError;
use margaret_authorization_grants_database::database_authorization_grants::DatabaseAuthorizationGrants;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_model::qualified_framework_table::qualified_framework_table;
use margaret_store_contract_tests::contract_instant::contract_instant;
use margaret_store_contract_tests::contract_issued_code::contract_issued_code;
use margaret_store_contract_tests::contract_pending_authorization::contract_pending_authorization;
use margaret_store_contract_tests::contract_refresh_family::contract_refresh_family;
use margaret_store_contract_tests::contract_token::contract_token;

use crate::framework_state_database::framework_state_database;

async fn corrupted(started: &StartedDatabase, table: &str, column: &str) {
    started
        .database
        .client()
        .await
        .expect("a connection is checked out")
        .execute(
            &format!(
                "UPDATE {} SET \"{column}\" = 'not json'",
                qualified_framework_table(table)
            ),
            &[],
        )
        .await
        .expect("the stored column is corrupted");
}

fn grants_error(error: &anyhow::Error) -> Option<&AuthorizationGrantsDatabaseError> {
    error.downcast_ref::<AuthorizationGrantsDatabaseError>()
}

#[tokio::test]
async fn reports_a_refresh_token_lookup_in_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;
    let error = DatabaseAuthorizationGrants::create(Arc::clone(&started.database))
        .find_refresh_token(contract_token())
        .await
        .expect_err("the lookup fails without the grant tables");

    assert!(matches!(
        grants_error(&error),
        Some(AuthorizationGrantsDatabaseError::FindRefreshToken(_))
    ));
}

#[tokio::test]
async fn reports_a_pending_authorization_held_in_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;
    let error = DatabaseAuthorizationGrants::create(Arc::clone(&started.database))
        .hold_pending_authorization(
            Uuid::new_v4(),
            contract_pending_authorization(),
            contract_instant(),
        )
        .await
        .expect_err("the hold fails without the grant tables");

    assert!(matches!(
        grants_error(&error),
        Some(AuthorizationGrantsDatabaseError::HoldPendingAuthorization(
            _
        ))
    ));
}

#[tokio::test]
async fn reports_a_code_issued_in_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;
    let error = DatabaseAuthorizationGrants::create(Arc::clone(&started.database))
        .issue_code(contract_token(), contract_issued_code(), contract_instant())
        .await
        .expect_err("the issue fails without the grant tables");

    assert!(matches!(
        grants_error(&error),
        Some(AuthorizationGrantsDatabaseError::IssueCode(_))
    ));
}

#[tokio::test]
async fn reports_a_refresh_family_opened_in_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;
    let error = DatabaseAuthorizationGrants::create(Arc::clone(&started.database))
        .open_refresh_family(
            Uuid::new_v4(),
            contract_refresh_family(),
            contract_token(),
            contract_instant(),
        )
        .await
        .expect_err("the opening fails without the grant tables");

    assert!(matches!(
        grants_error(&error),
        Some(AuthorizationGrantsDatabaseError::OpenRefreshFamily(_))
    ));
}

#[tokio::test]
async fn reports_a_code_redeemed_in_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;
    let error = DatabaseAuthorizationGrants::create(Arc::clone(&started.database))
        .redeem_code(contract_token(), Uuid::new_v4())
        .await
        .expect_err("the redemption fails without the grant tables");

    assert!(matches!(
        grants_error(&error),
        Some(AuthorizationGrantsDatabaseError::RedeemCode(_))
    ));
}

#[tokio::test]
async fn reports_a_refresh_family_revoked_in_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;
    let error = DatabaseAuthorizationGrants::create(Arc::clone(&started.database))
        .revoke_refresh_family(Uuid::new_v4(), contract_instant())
        .await
        .expect_err("the revocation fails without the grant tables");

    assert!(matches!(
        grants_error(&error),
        Some(AuthorizationGrantsDatabaseError::RevokeRefreshFamily(_))
    ));
}

#[tokio::test]
async fn reports_a_refresh_token_rotated_in_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;
    let error = DatabaseAuthorizationGrants::create(Arc::clone(&started.database))
        .rotate_refresh_token(contract_token(), contract_token(), contract_instant())
        .await
        .expect_err("the rotation fails without the grant tables");

    assert!(matches!(
        grants_error(&error),
        Some(AuthorizationGrantsDatabaseError::RotateRefreshToken(_))
    ));
}

#[tokio::test]
async fn reports_a_pending_authorization_taken_from_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;
    let error = DatabaseAuthorizationGrants::create(Arc::clone(&started.database))
        .take_pending_authorization(Uuid::new_v4())
        .await
        .expect_err("the take fails without the grant tables");

    assert!(matches!(
        grants_error(&error),
        Some(AuthorizationGrantsDatabaseError::TakePendingAuthorization(
            _
        ))
    ));
}

#[tokio::test]
async fn reports_a_malformed_stored_grant() {
    let started = framework_state_database().await;
    let store = DatabaseAuthorizationGrants::create(Arc::clone(&started.database));
    let id = Uuid::new_v4();

    store
        .hold_pending_authorization(id, contract_pending_authorization(), contract_instant())
        .await
        .expect("the store holds the pending authorization");
    corrupted(&started, "pending_authorizations", "grant").await;

    let error = store
        .take_pending_authorization(id)
        .await
        .expect_err("the take refuses a malformed grant");

    assert!(matches!(
        grants_error(&error),
        Some(AuthorizationGrantsDatabaseError::MalformedGrant(_))
    ));
}

#[tokio::test]
async fn reports_malformed_stored_scopes() {
    let started = framework_state_database().await;
    let store = DatabaseAuthorizationGrants::create(Arc::clone(&started.database));
    let first_token = contract_token();

    store
        .open_refresh_family(
            Uuid::new_v4(),
            contract_refresh_family(),
            first_token,
            contract_instant(),
        )
        .await
        .expect("the store opens the refresh family");
    corrupted(&started, "refresh_families", "scopes").await;

    let error = store
        .find_refresh_token(first_token)
        .await
        .expect_err("the lookup refuses malformed scopes");

    assert!(matches!(
        grants_error(&error),
        Some(AuthorizationGrantsDatabaseError::MalformedScopes(_))
    ));
}
