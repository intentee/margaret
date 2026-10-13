use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use margaret::framework::macros::model;
use margaret::framework::model::column_default::ColumnDefault;

use crate::models::article::Article;
use crate::models::profile::Profile;

#[model(table = "authors")]
#[has_many(name = "articles", model = Article, key = author)]
#[has_one(name = "profile", model = Profile, key = author)]
#[derive(Clone, Debug, PartialEq)]
pub struct Author {
    #[column(primary_key, default = ColumnDefault::UuidV7)]
    pub id: Uuid,
    #[column(unique)]
    pub name: String,
    #[column]
    pub joined_at: DateTime<Utc>,
    #[column]
    pub bio: Option<String>,
}
