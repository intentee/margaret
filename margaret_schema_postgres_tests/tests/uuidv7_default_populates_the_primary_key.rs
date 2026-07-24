use chrono::Utc;
use sqlx::query_scalar;
use uuid::Uuid;

use margaret_example::margaret::schema::schema;

use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::start_database::start_database;

#[tokio::test]
async fn the_uuidv7_default_populates_an_omitted_primary_key() {
    let database = start_database().await;
    let pool = database.pool();

    apply_schema(pool, &schema()).await;

    let generated_id: Uuid = query_scalar(
        "INSERT INTO authors (name, is_active, joined_at, bio) VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind("Grace Hopper")
    .bind(true)
    .bind(Utc::now())
    .bind(None::<String>)
    .fetch_one(pool)
    .await
    .expect("the author is inserted with a defaulted primary key");

    assert_eq!(generated_id.get_version_num(), 7);
}
