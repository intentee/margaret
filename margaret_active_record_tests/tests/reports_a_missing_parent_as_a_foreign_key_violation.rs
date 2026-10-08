use std::collections::BTreeMap;

use rust_decimal::Decimal;
use uuid::Uuid;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::active_record::json::Json;
use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::statement_kind::StatementKind;
use margaret_active_record_tests::margaret::models::models_article_article::draft::Draft;
use margaret_active_record_tests::models::article::Article;
use margaret_active_record_tests::models::article_status::ArticleStatus;

use crate::started_with_models::started_with_models;

#[tokio::test]
async fn reports_a_missing_parent_as_a_foreign_key_violation() {
    let started = started_with_models().await;

    assert!(matches!(
        Article::create(Draft {
            title: "Orphan".to_string(),
            price: Decimal::ZERO,
            status: ArticleStatus::Draft,
            cover: None,
            tags: Json::new(BTreeMap::new()),
            author: Key::new(Uuid::nil()),
        })
        .run(started.database.as_ref())
        .await,
        Err(ActiveRecordError::ForeignKeyViolation {
            statement: StatementKind::Insert,
            table: "articles",
            ..
        })
    ));
}
