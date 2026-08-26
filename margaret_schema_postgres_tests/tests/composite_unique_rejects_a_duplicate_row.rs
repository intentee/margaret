use sqlx::PgPool;
use sqlx::query;
use uuid::Uuid;

use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::start_database::start_database;

const UNIQUE_VIOLATION: &str = "23505";

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
async fn a_second_fragment_sharing_a_hash_and_context_is_rejected() {
    let database = start_database().await;
    let pool = database.pool();

    apply_schema(pool).await;

    let hash = vec![5u8; 32];
    let context = Uuid::new_v4();
    let first_partition = Uuid::new_v4();
    let second_partition = Uuid::new_v4();

    insert_fragment_metadata(pool, first_partition, hash.clone()).await;
    insert_fragment_metadata(pool, second_partition, hash.clone()).await;

    query(INSERT_FRAGMENT)
        .bind(first_partition)
        .bind(hash.clone())
        .bind(context)
        .execute(pool)
        .await
        .expect("the first fragment is accepted");

    let rejection = query(INSERT_FRAGMENT)
        .bind(second_partition)
        .bind(hash)
        .bind(context)
        .execute(pool)
        .await
        .expect_err("a second fragment sharing the unique columns is rejected");

    assert_eq!(
        rejection
            .as_database_error()
            .expect("the rejection carries a database error")
            .code()
            .expect("the database error carries an SQLSTATE"),
        UNIQUE_VIOLATION
    );
}
