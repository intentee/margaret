use std::collections::BTreeMap;

use rust_decimal::Decimal;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::active_record::json::Json;
use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::statement_kind::StatementKind;
use margaret_active_record_tests::margaret::models::models_article_article::draft::Draft;
use margaret_active_record_tests::models::article::Article;
use margaret_active_record_tests::models::article_status::ArticleStatus;

use crate::created_author::created_author;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn reports_a_price_beyond_its_precision_as_out_of_range() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let author = created_author(database, "Milo").await;

    assert!(matches!(
        Article::create(Draft {
            title: "Shipping".to_string(),
            price: Decimal::new(100_000_000_000, 0),
            status: ArticleStatus::Draft,
            cover: None,
            tags: Json::new(BTreeMap::new()),
            author: Key::of(&author),
        })
        .run(database)
        .await,
        Err(ActiveRecordError::NumericValueOutOfRange {
            statement: StatementKind::Insert,
            table: "articles",
            ..
        })
    ));
}
