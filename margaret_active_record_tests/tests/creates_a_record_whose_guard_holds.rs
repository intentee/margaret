use std::collections::BTreeMap;

use rust_decimal::Decimal;

use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::active_record::creation::Creation;
use margaret::framework::active_record::json::Json;
use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::margaret::models::models_article_article::draft::Draft;
use margaret_active_record_tests::models::article::Article;
use margaret_active_record_tests::models::article_status::ArticleStatus;
use margaret_active_record_tests::models::author::Author;

use crate::created_author::created_author;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn creates_a_record_whose_guard_holds() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let author = created_author(database, "Milo").await;
    let creation = Article::create(Draft {
        title: "Shipping".to_string(),
        price: Decimal::ZERO,
        status: ArticleStatus::Draft,
        cover: None,
        tags: Json::new(BTreeMap::new()),
        author: Key::of(&author),
    })
    .when(Author::query().id.eq(author.id).exists())
    .run(database)
    .await
    .expect("the guarded creation completes");

    assert!(matches!(
        creation,
        Creation::Created(Article { ref title, .. }) if title == "Shipping"
    ));
}
