use sqlx::Error;
use sqlx::PgPool;
use sqlx::query;
use uuid::Uuid;

use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::start_database::start_database;

const CHECK_VIOLATION: &str = "23514";

const INSERT_FRAGMENT_METADATA: &str =
    "INSERT INTO fragment_metadata (partition, hash, size_payload) VALUES ($1, $2, $3)";

async fn insert_fragment_metadata(
    pool: &PgPool,
    hash: Vec<u8>,
    size_payload: i64,
) -> Result<(), Error> {
    query(INSERT_FRAGMENT_METADATA)
        .bind(Uuid::new_v4())
        .bind(hash)
        .bind(size_payload)
        .execute(pool)
        .await
        .map(|_| ())
}

fn sqlstate(error: &Error) -> String {
    error
        .as_database_error()
        .expect("the rejection carries a database error")
        .code()
        .expect("the database error carries an SQLSTATE")
        .to_string()
}

#[tokio::test]
async fn a_hash_of_the_declared_byte_length_is_accepted() {
    let database = start_database().await;
    let pool = database.pool();

    apply_schema(pool).await;

    insert_fragment_metadata(pool, vec![0u8; 32], 0)
        .await
        .expect("a thirty two byte hash is accepted");
}

#[tokio::test]
async fn a_hash_shorter_than_the_declared_byte_length_is_rejected() {
    let database = start_database().await;
    let pool = database.pool();

    apply_schema(pool).await;

    let rejection = insert_fragment_metadata(pool, vec![0u8; 16], 0)
        .await
        .expect_err("a sixteen byte hash is rejected");

    assert_eq!(sqlstate(&rejection), CHECK_VIOLATION);
}

#[tokio::test]
async fn a_hash_longer_than_the_declared_byte_length_is_rejected() {
    let database = start_database().await;
    let pool = database.pool();

    apply_schema(pool).await;

    let rejection = insert_fragment_metadata(pool, vec![0u8; 33], 0)
        .await
        .expect_err("a thirty three byte hash is rejected");

    assert_eq!(sqlstate(&rejection), CHECK_VIOLATION);
}

#[tokio::test]
async fn a_size_below_the_declared_minimum_is_rejected() {
    let database = start_database().await;
    let pool = database.pool();

    apply_schema(pool).await;

    let rejection = insert_fragment_metadata(pool, vec![0u8; 32], -1)
        .await
        .expect_err("a negative size is rejected");

    assert_eq!(sqlstate(&rejection), CHECK_VIOLATION);
}
