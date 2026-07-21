use chrono::DateTime;
use chrono::Utc;

use margaret_macros::model;

use crate::models::author::Author;

#[model(table = "articles")]
#[derive(Clone)]
pub struct Article {
    #[column(primary_key)]
    pub id: uuid::Uuid,
    #[column]
    pub title: String,
    #[column]
    pub body: String,
    #[column]
    pub cover: Option<Vec<u8>>,
    #[column]
    pub published: bool,
    #[column]
    #[index]
    pub created_at: DateTime<Utc>,
    #[column]
    #[foreign_key(on_delete = cascade)]
    #[index]
    pub author: Author,
}
