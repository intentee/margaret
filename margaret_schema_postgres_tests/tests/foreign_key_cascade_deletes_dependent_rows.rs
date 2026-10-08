use chrono::Utc;
use uuid::Uuid;

use margaret_schema_postgres_fixture::margaret::schema::schema;
use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::started_database::StartedDatabase;

#[tokio::test]
async fn deleting_an_author_cascades_to_its_articles() {
    let started = StartedDatabase::start().await;

    apply_schema(&started.database, &schema()).await;

    let client = started
        .database
        .client()
        .await
        .expect("a connection is checked out");
    let author_id = Uuid::new_v4();

    client
        .execute(
            "INSERT INTO authors (id, name, is_active, joined_at, bio) VALUES ($1, $2, $3, $4, $5)",
            &[
                &author_id,
                &"Edsger Dijkstra",
                &true,
                &Utc::now(),
                &None::<String>,
            ],
        )
        .await
        .expect("the author is inserted");
    client
        .execute(
            "INSERT INTO articles (id, title, body, cover, published, status, created_at, author_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            &[
                &Uuid::new_v4(),
                &"A Discipline of Programming",
                &"the body text",
                &None::<Vec<u8>>,
                &true,
                &"Published",
                &Utc::now(),
                &author_id,
            ],
        )
        .await
        .expect("the article is inserted");
    client
        .execute("DELETE FROM authors WHERE id = $1", &[&author_id])
        .await
        .expect("the author is deleted");

    assert_eq!(
        client
            .query_one(
                "SELECT COUNT(*) FROM articles WHERE author_id = $1",
                &[&author_id],
            )
            .await
            .expect("the remaining article count is read")
            .get::<_, i64>(0),
        0
    );
}
