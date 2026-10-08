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
async fn a_second_fragment_sharing_a_hash_and_context_is_rejected() {
    let started = StartedDatabase::start().await;

    apply_schema(&started.database, &schema()).await;

    let client = started
        .database
        .client()
        .await
        .expect("a connection is checked out");
    let hash = vec![5u8; 32];
    let context = Uuid::new_v4();
    let first_partition = Uuid::new_v4();
    let second_partition = Uuid::new_v4();

    insert_fragment_metadata(&client, first_partition, &hash).await;
    insert_fragment_metadata(&client, second_partition, &hash).await;

    client
        .execute(INSERT_FRAGMENT, &[&first_partition, &hash, &context])
        .await
        .expect("the first fragment is accepted");

    let rejection = client
        .execute(INSERT_FRAGMENT, &[&second_partition, &hash, &context])
        .await
        .expect_err("a second fragment sharing the unique columns is rejected");

    assert_eq!(rejection.code(), Some(&SqlState::UNIQUE_VIOLATION));
}
