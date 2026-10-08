use chrono::Utc;
use uuid::Uuid;

use margaret_schema_postgres_fixture::margaret::schema::schema;
use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::started_database::StartedDatabase;

#[tokio::test]
async fn applies_the_generated_schema_and_round_trips_a_model() {
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
                &"Ada Lovelace",
                &true,
                &Utc::now(),
                &None::<String>,
            ],
        )
        .await
        .expect("the author row is inserted");

    let article_id = Uuid::new_v4();
    let cover: Vec<u8> = vec![0x89, 0x50, 0x4e, 0x47];

    client
        .execute(
            "INSERT INTO articles (id, title, body, cover, published, status, created_at, author_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            &[
                &article_id,
                &"On Computable Numbers",
                &"the body text",
                &cover,
                &true,
                &"Published",
                &Utc::now(),
                &author_id,
            ],
        )
        .await
        .expect("the article row is inserted");

    let article = client
        .query_one(
            "SELECT title, body, cover, published, author_id FROM articles WHERE id = $1",
            &[&article_id],
        )
        .await
        .expect("the article row is read back");

    assert_eq!(article.get::<_, String>("title"), "On Computable Numbers");
    assert_eq!(article.get::<_, String>("body"), "the body text");
    assert_eq!(article.get::<_, Option<Vec<u8>>>("cover"), Some(cover));
    assert!(article.get::<_, bool>("published"));
    assert_eq!(article.get::<_, Uuid>("author_id"), author_id);
}
