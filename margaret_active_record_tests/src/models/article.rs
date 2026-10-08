use std::collections::BTreeMap;

use rust_decimal::Decimal;
use uuid::Uuid;

use margaret::framework::active_record::json::Json;
use margaret::framework::active_record::key::Key;
use margaret::framework::macros::model;
use margaret::framework::model::on_delete::OnDelete;

use crate::models::article_status::ArticleStatus;
use crate::models::article_translation::ArticleTranslation;
use crate::models::author::Author;

#[model(table = "articles")]
#[index(name = "articles_by_author", fields = [author, id])]
#[index(name = "articles_by_author_title", fields = [author, title])]
#[has_many(name = "translations", model = ArticleTranslation, key = article)]
#[derive(Clone, Debug, PartialEq)]
pub struct Article {
    #[column(primary_key)]
    pub id: Uuid,
    #[column]
    pub title: String,
    #[column(precision = 12, scale = 2)]
    pub price: Decimal,
    #[column]
    pub status: ArticleStatus,
    #[column]
    pub cover: Option<Vec<u8>>,
    #[column]
    pub tags: Json<BTreeMap<String, String>>,
    #[column]
    #[foreign_key(on_delete = OnDelete::Cascade)]
    pub author: Key<Author>,
}
