use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::article::Article;

use crate::postgres::created_article::created_article;
use crate::postgres::created_author::created_author;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn reads_the_payload_of_a_json_column() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let author = created_author(database, "Milo").await;
    let article = created_article(database, &author, "Shipping").await;

    started
        .administration
        .execute(r#"UPDATE articles SET tags = '{"topic":"release"}'"#)
        .await;

    let Lookup::Found(found) = Article::query()
        .id
        .eq(article.id)
        .find(database)
        .await
        .expect("the article is read")
    else {
        panic!("the article is found");
    };

    assert_eq!(
        found.tags.payload().get("topic").map(String::as_str),
        Some("release")
    );
}
