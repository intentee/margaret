use chrono::Utc;
use sqlx::query;
use sqlx::query_as;
use uuid::Uuid;

use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::start_database::start_database;

#[tokio::test]
async fn applies_the_generated_schema_and_round_trips_a_model() {
    let database = start_database().await;
    let pool = database.pool();

    apply_schema(pool).await;

    let author_id = Uuid::new_v4();

    query("INSERT INTO authors (id, name, is_active, joined_at, bio) VALUES ($1, $2, $3, $4, $5)")
        .bind(author_id)
        .bind("Ada Lovelace")
        .bind(true)
        .bind(Utc::now())
        .bind(None::<String>)
        .execute(pool)
        .await
        .expect("the author row is inserted");

    let article_id = Uuid::new_v4();
    let cover: Vec<u8> = vec![0x89, 0x50, 0x4e, 0x47];

    query(
        "INSERT INTO articles (id, title, body, cover, published, created_at, author_id) VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(article_id)
    .bind("On Computable Numbers")
    .bind("the body text")
    .bind(&cover)
    .bind(true)
    .bind(Utc::now())
    .bind(author_id)
    .execute(pool)
    .await
    .expect("the article row is inserted");

    let (title, body, stored_cover, published, stored_author_id): (
        String,
        String,
        Option<Vec<u8>>,
        bool,
        Uuid,
    ) = query_as("SELECT title, body, cover, published, author_id FROM articles WHERE id = $1")
        .bind(article_id)
        .fetch_one(pool)
        .await
        .expect("the article row is read back");

    assert_eq!(title, "On Computable Numbers");
    assert_eq!(body, "the body text");
    assert_eq!(stored_cover, Some(cover));
    assert!(published);
    assert_eq!(stored_author_id, author_id);
}
