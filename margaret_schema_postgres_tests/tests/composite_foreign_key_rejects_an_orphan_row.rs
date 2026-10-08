use tokio_postgres::Client;
use tokio_postgres::error::SqlState;
use uuid::Uuid;

use margaret_database_tests::apply_schema::apply_schema;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_schema_postgres_fixture::margaret::schema::schema;

const INSERT_FRAGMENT: &str = "INSERT INTO fragment (partition, hash, context) VALUES ($1, $2, $3)";

async fn insert_fragment_metadata(client: &Client, partition: Uuid, hash: &[u8]) {
    client
        .execute(
            "INSERT INTO fragment_metadata (partition, hash, size_payload) VALUES ($1, $2, $3)",
            &[&partition, &hash, &0_i64],
        )
        .await
        .expect("the fragment metadata is inserted");
}

#[tokio::test]
async fn a_fragment_with_matching_metadata_is_accepted() {
    let started = StartedDatabase::start().await;

    apply_schema(&started.database, &schema()).await;

    let client = started
        .database
        .client()
        .await
        .expect("a connection is checked out");
    let partition = Uuid::new_v4();
    let hash = vec![7u8; 32];

    insert_fragment_metadata(&client, partition, &hash).await;

    client
        .execute(INSERT_FRAGMENT, &[&partition, &hash, &Uuid::new_v4()])
        .await
        .expect("a fragment backed by its metadata is accepted");
}

#[tokio::test]
async fn a_fragment_whose_hash_has_no_metadata_is_rejected() {
    let started = StartedDatabase::start().await;

    apply_schema(&started.database, &schema()).await;

    let client = started
        .database
        .client()
        .await
        .expect("a connection is checked out");
    let partition = Uuid::new_v4();

    insert_fragment_metadata(&client, partition, &[7u8; 32]).await;

    let rejection = client
        .execute(
            INSERT_FRAGMENT,
            &[&partition, &vec![9u8; 32], &Uuid::new_v4()],
        )
        .await
        .expect_err("a fragment without matching metadata is rejected");

    assert_eq!(rejection.code(), Some(&SqlState::FOREIGN_KEY_VIOLATION));
}

#[tokio::test]
async fn a_fragment_whose_partition_has_no_metadata_is_rejected() {
    let started = StartedDatabase::start().await;

    apply_schema(&started.database, &schema()).await;

    let client = started
        .database
        .client()
        .await
        .expect("a connection is checked out");
    let hash = vec![7u8; 32];

    insert_fragment_metadata(&client, Uuid::new_v4(), &hash).await;

    let rejection = client
        .execute(INSERT_FRAGMENT, &[&Uuid::new_v4(), &hash, &Uuid::new_v4()])
        .await
        .expect_err("a fragment in another partition is rejected");

    assert_eq!(rejection.code(), Some(&SqlState::FOREIGN_KEY_VIOLATION));
}
