use chrono::DateTime;
use chrono::Utc;

use margaret_macros::model;

#[model(table = "authors")]
#[derive(Clone)]
pub struct Author {
    #[column(primary_key)]
    pub id: uuid::Uuid,
    #[column]
    pub name: String,
    #[column(name = "is_active")]
    pub active: bool,
    #[column]
    pub joined_at: DateTime<Utc>,
    #[column]
    pub bio: Option<String>,
}
