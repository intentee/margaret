use std::collections::BTreeMap;

use rust_decimal::Decimal;

use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::active_record::json::Json;
use margaret::framework::active_record::key::Key;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::margaret::models::models_article_article::draft::Draft;
use margaret_active_record_tests::models::article::Article;
use margaret_active_record_tests::models::article_status::ArticleStatus;
use margaret_active_record_tests::models::author::Author;

pub async fn created_article(database: &Database, author: &Author, title: &str) -> Article {
    Article::create(Draft {
        title: title.to_string(),
        price: Decimal::new(499, 2),
        status: ArticleStatus::Draft,
        cover: None,
        tags: Json::new(BTreeMap::new()),
        author: Key::of(author),
    })
    .run(database)
    .await
    .expect("the article is created")
}
