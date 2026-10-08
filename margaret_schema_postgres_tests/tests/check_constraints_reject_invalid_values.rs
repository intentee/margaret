use uuid::Uuid;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::statement_kind::StatementKind;
use margaret::framework::database::database::Database;
use margaret_schema_postgres_fixture::fragment_metadata::FragmentMetadata;

use crate::started_with_fixture::started_with_fixture;

async fn insert_fragment_metadata(
    database: &Database,
    hash: Vec<u8>,
    size_payload: i64,
) -> Result<(), ActiveRecordError> {
    FragmentMetadata {
        partition: Uuid::new_v4(),
        hash,
        size_payload,
    }
    .insert()
    .run(database)
    .await
}

fn is_check_violation(outcome: &Result<(), ActiveRecordError>) -> bool {
    matches!(
        outcome,
        Err(ActiveRecordError::CheckViolation {
            statement: StatementKind::Insert,
            table: "fragment_metadata",
            ..
        })
    )
}

#[tokio::test]
async fn a_hash_of_the_declared_byte_length_is_accepted() {
    let started = started_with_fixture().await;

    insert_fragment_metadata(&started.database, vec![0u8; 32], 0)
        .await
        .expect("a thirty two byte hash is accepted");
}

#[tokio::test]
async fn a_hash_shorter_than_the_declared_byte_length_is_rejected() {
    let started = started_with_fixture().await;

    assert!(is_check_violation(
        &insert_fragment_metadata(&started.database, vec![0u8; 16], 0).await
    ));
}

#[tokio::test]
async fn a_hash_longer_than_the_declared_byte_length_is_rejected() {
    let started = started_with_fixture().await;

    assert!(is_check_violation(
        &insert_fragment_metadata(&started.database, vec![0u8; 33], 0).await
    ));
}

#[tokio::test]
async fn a_size_below_the_declared_minimum_is_rejected() {
    let started = started_with_fixture().await;

    assert!(is_check_violation(
        &insert_fragment_metadata(&started.database, vec![0u8; 32], -1).await
    ));
}
