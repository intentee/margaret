use tokio_postgres::Client;
use tokio_postgres::Error;
use tokio_postgres::error::SqlState;
use uuid::Uuid;

use margaret_schema_postgres_fixture::margaret::schema::schema;
use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::started_database::StartedDatabase;

async fn insert_fragment_metadata(
    client: &Client,
    hash: Vec<u8>,
    size_payload: i64,
) -> Result<u64, Error> {
    client
        .execute(
            "INSERT INTO fragment_metadata (partition, hash, size_payload) VALUES ($1, $2, $3)",
            &[&Uuid::new_v4(), &hash, &size_payload],
        )
        .await
}

#[tokio::test]
async fn a_hash_of_the_declared_byte_length_is_accepted() {
    let started = StartedDatabase::start().await;

    apply_schema(&started.database, &schema()).await;

    let client = started
        .database
        .client()
        .await
        .expect("a connection is checked out");

    insert_fragment_metadata(&client, vec![0u8; 32], 0)
        .await
        .expect("a thirty two byte hash is accepted");
}

#[tokio::test]
async fn a_hash_shorter_than_the_declared_byte_length_is_rejected() {
    let started = StartedDatabase::start().await;

    apply_schema(&started.database, &schema()).await;

    let client = started
        .database
        .client()
        .await
        .expect("a connection is checked out");

    let rejection = insert_fragment_metadata(&client, vec![0u8; 16], 0)
        .await
        .expect_err("a sixteen byte hash is rejected");

    assert_eq!(rejection.code(), Some(&SqlState::CHECK_VIOLATION));
}

#[tokio::test]
async fn a_hash_longer_than_the_declared_byte_length_is_rejected() {
    let started = StartedDatabase::start().await;

    apply_schema(&started.database, &schema()).await;

    let client = started
        .database
        .client()
        .await
        .expect("a connection is checked out");

    let rejection = insert_fragment_metadata(&client, vec![0u8; 33], 0)
        .await
        .expect_err("a thirty three byte hash is rejected");

    assert_eq!(rejection.code(), Some(&SqlState::CHECK_VIOLATION));
}

#[tokio::test]
async fn a_size_below_the_declared_minimum_is_rejected() {
    let started = StartedDatabase::start().await;

    apply_schema(&started.database, &schema()).await;

    let client = started
        .database
        .client()
        .await
        .expect("a connection is checked out");

    let rejection = insert_fragment_metadata(&client, vec![0u8; 32], -1)
        .await
        .expect_err("a negative size is rejected");

    assert_eq!(rejection.code(), Some(&SqlState::CHECK_VIOLATION));
}
