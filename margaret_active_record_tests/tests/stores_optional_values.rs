use std::collections::BTreeMap;

use rust_decimal::Decimal;

use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::active_record::json::Json;
use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::margaret::models::models_article_article::draft::Draft;
use margaret_active_record_tests::models::article::Article;
use margaret_active_record_tests::models::article_status::ArticleStatus;

use crate::created_author::created_author;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn stores_optional_values() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let author = created_author(database, "Milo").await;
    let mut tags = BTreeMap::new();

    tags.insert("topic".to_string(), "release".to_string());

    let article = Article::create(Draft {
        title: "Shipping".to_string(),
        price: Decimal::new(1_250, 2),
        status: ArticleStatus::Published,
        cover: Some(vec![1, 2, 3]),
        tags: Json::new(tags),
        author: Key::of(&author),
    })
    .run(database)
    .await
    .expect("the article is created");

    assert_eq!(
        Article::query()
            .id
            .eq(article.id)
            .find(database)
            .await
            .expect("the article is read"),
        Lookup::Found(article)
    );
}
