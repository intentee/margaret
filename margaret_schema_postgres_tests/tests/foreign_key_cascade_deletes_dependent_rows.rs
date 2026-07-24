use chrono::Utc;
use sqlx::query;
use sqlx::query_scalar;
use uuid::Uuid;

use margaret_example::margaret::schema::schema;

use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::start_database::start_database;

#[tokio::test]
async fn deleting_an_author_cascades_to_its_articles() {
    let database = start_database().await;
    let pool = database.pool();

    apply_schema(pool, &schema()).await;

    let author_id = Uuid::new_v4();

    query("INSERT INTO authors (id, name, is_active, joined_at, bio) VALUES ($1, $2, $3, $4, $5)")
        .bind(author_id)
        .bind("Edsger Dijkstra")
        .bind(true)
        .bind(Utc::now())
        .bind(None::<String>)
        .execute(pool)
        .await
        .expect("the author is inserted");

    query(
        "INSERT INTO articles (id, title, body, cover, published, created_at, author_id) VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(Uuid::new_v4())
    .bind("A Discipline of Programming")
    .bind("the body text")
    .bind(None::<Vec<u8>>)
    .bind(true)
    .bind(Utc::now())
    .bind(author_id)
    .execute(pool)
    .await
    .expect("the article is inserted");

    query("DELETE FROM authors WHERE id = $1")
        .bind(author_id)
        .execute(pool)
        .await
        .expect("the author is deleted");

    let remaining: i64 = query_scalar("SELECT COUNT(*) FROM articles WHERE author_id = $1")
        .bind(author_id)
        .fetch_one(pool)
        .await
        .expect("the remaining article count is read");

    assert_eq!(remaining, 0);
}
