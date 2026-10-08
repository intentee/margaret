use chrono::DateTime;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use margaret::framework::macros::model;
use margaret::framework::model::on_delete::OnDelete;

use crate::models::article_status::ArticleStatus;
use crate::models::author::Author;

#[model(table = "articles")]
#[derive(Clone)]
pub struct Article {
    #[column(primary_key)]
    pub id: Uuid,
    #[column]
    pub title: String,
    #[column]
    pub body: String,
    #[column]
    pub cover: Option<Vec<u8>>,
    #[column]
    pub published: bool,
    #[column(precision = 12, scale = 2)]
    pub price: Decimal,
    #[column(minimum = 0)]
    pub reading_minutes: f64,
    #[column]
    pub status: ArticleStatus,
    #[column]
    #[index]
    pub created_at: DateTime<Utc>,
    #[column]
    #[foreign_key(on_delete = OnDelete::Cascade)]
    #[index]
    pub author: Author,
}
