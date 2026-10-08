use std::collections::BTreeMap;

use crate::created_article::created_article;
use crate::created_author::created_author;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn takes_the_payload_of_a_json_column() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let author = created_author(database, "Milo").await;

    assert_eq!(
        created_article(database, &author, "Shipping")
            .await
            .tags
            .into_payload(),
        BTreeMap::new()
    );
}
