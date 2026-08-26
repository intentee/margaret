use sqlx::PgPool;
use sqlx::query;
use uuid::Uuid;

use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::start_database::start_database;

const FOREIGN_KEY_VIOLATION: &str = "23503";

const INSERT_FRAGMENT: &str = "INSERT INTO fragment (partition, hash, context) VALUES ($1, $2, $3)";

async fn insert_fragment_metadata(pool: &PgPool, partition: Uuid, hash: Vec<u8>) {
    query("INSERT INTO fragment_metadata (partition, hash, size_payload) VALUES ($1, $2, $3)")
        .bind(partition)
        .bind(hash)
        .bind(0i64)
        .execute(pool)
        .await
        .expect("the fragment metadata is inserted");
}

#[tokio::test]
async fn a_fragment_with_matching_metadata_is_accepted() {
    let database = start_database().await;
    let pool = database.pool();

    apply_schema(pool).await;

    let partition = Uuid::new_v4();
    let hash = vec![7u8; 32];

    insert_fragment_metadata(pool, partition, hash.clone()).await;

    query(INSERT_FRAGMENT)
        .bind(partition)
        .bind(hash)
        .bind(Uuid::new_v4())
        .execute(pool)
        .await
        .expect("a fragment backed by its metadata is accepted");
}

#[tokio::test]
async fn a_fragment_whose_hash_has_no_metadata_is_rejected() {
    let database = start_database().await;
    let pool = database.pool();

    apply_schema(pool).await;

    let partition = Uuid::new_v4();

    insert_fragment_metadata(pool, partition, vec![7u8; 32]).await;

    let rejection = query(INSERT_FRAGMENT)
        .bind(partition)
        .bind(vec![9u8; 32])
        .bind(Uuid::new_v4())
        .execute(pool)
        .await
        .expect_err("a fragment without matching metadata is rejected");

    assert_eq!(
        rejection
            .as_database_error()
            .expect("the rejection carries a database error")
            .code()
            .expect("the database error carries an SQLSTATE"),
        FOREIGN_KEY_VIOLATION
    );
}

#[tokio::test]
async fn a_fragment_whose_partition_has_no_metadata_is_rejected() {
    let database = start_database().await;
    let pool = database.pool();

    apply_schema(pool).await;

    let hash = vec![7u8; 32];

    insert_fragment_metadata(pool, Uuid::new_v4(), hash.clone()).await;

    let rejection = query(INSERT_FRAGMENT)
        .bind(Uuid::new_v4())
        .bind(hash)
        .bind(Uuid::new_v4())
        .execute(pool)
        .await
        .expect_err("a fragment in another partition is rejected");

    assert_eq!(
        rejection
            .as_database_error()
            .expect("the rejection carries a database error")
            .code()
            .expect("the database error carries an SQLSTATE"),
        FOREIGN_KEY_VIOLATION
    );
}
