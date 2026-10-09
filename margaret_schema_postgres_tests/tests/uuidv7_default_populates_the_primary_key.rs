use chrono::Utc;
use uuid::Uuid;

use margaret_database_tests::apply_schema::apply_schema;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_schema_postgres_fixture::margaret::schema::schema;

#[tokio::test]
async fn the_uuidv7_default_populates_an_omitted_primary_key() {
    let started = StartedDatabase::start().await;

    apply_schema(&started.database, &schema()).await;

    let client = started
        .database
        .client()
        .await
        .expect("a connection is checked out");

    let generated_id: Uuid = client
        .query_one(
            "INSERT INTO authors (name, is_active, joined_at, bio) VALUES ($1, $2, $3, $4) RETURNING id",
            &[&"Grace Hopper", &true, &Utc::now(), &None::<String>],
        )
        .await
        .expect("the author is inserted with a defaulted primary key")
        .get("id");

    assert_eq!(generated_id.get_version_num(), 7);
}
