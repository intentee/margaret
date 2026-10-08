use uuid::Uuid;

use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_schema_postgres_fixture::article::Article;
use margaret_schema_postgres_fixture::article_status::ArticleStatus;

use crate::inserted_author::inserted_author;
use crate::joined_at::joined_at;
use crate::started_with_fixture::started_with_fixture;

#[tokio::test]
async fn applies_the_generated_schema_and_round_trips_a_model() {
    let started = started_with_fixture().await;
    let database = started.database.as_ref();
    let author = inserted_author(database, "Ada Lovelace").await;
    let article = Article {
        id: Uuid::new_v4(),
        title: "On Computable Numbers".to_string(),
        body: "the body text".to_string(),
        cover: Some(vec![0x89, 0x50, 0x4e, 0x47]),
        published: true,
        status: ArticleStatus::Published,
        created_at: joined_at(),
        author: Key::of(&author),
    };

    article
        .insert()
        .run(database)
        .await
        .expect("the article is inserted");

    assert_eq!(
        Article::query()
            .id
            .eq(article.id)
            .find(database)
            .await
            .expect("the article is read back"),
        Lookup::Found(article)
    );
}
