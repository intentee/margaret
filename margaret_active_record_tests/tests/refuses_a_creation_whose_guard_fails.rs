use std::collections::BTreeMap;

use rust_decimal::Decimal;
use uuid::Uuid;

use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::active_record::creation::Creation;
use margaret::framework::active_record::json::Json;
use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::margaret::models::models_article_article::draft::Draft;
use margaret_active_record_tests::models::article::Article;
use margaret_active_record_tests::models::article_status::ArticleStatus;
use margaret_active_record_tests::models::author::Author;

use crate::started_with_models::started_with_models;

#[tokio::test]
async fn refuses_a_creation_whose_guard_fails() {
    let started = started_with_models().await;

    assert_eq!(
        Article::create(Draft {
            title: "Orphan".to_string(),
            price: Decimal::ZERO,
            status: ArticleStatus::Draft,
            cover: None,
            tags: Json::new(BTreeMap::new()),
            author: Key::new(Uuid::nil()),
        })
        .when(Author::query().id.eq(Uuid::nil()).exists())
        .run(started.database.as_ref())
        .await
        .expect("the guarded creation completes"),
        Creation::Refused
    );
}
