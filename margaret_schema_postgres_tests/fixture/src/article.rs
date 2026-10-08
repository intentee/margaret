use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use margaret::framework::macros::model;
use margaret::framework::model::on_delete::OnDelete;

use crate::article_status::ArticleStatus;
use crate::author::Author;

#[model(table = "articles")]
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
