use crate::postgres::created_article::created_article;
use crate::postgres::created_author::created_author;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn takes_the_primary_key_a_key_references() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let author = created_author(database, "Milo").await;

    assert_eq!(
        created_article(database, &author, "Shipping")
            .await
            .author
            .into_primary_key(),
        author.id
    );
}
