use chrono::DateTime;
use chrono::Utc;

use margaret::framework::macros::model;

#[model(table = "authors")]
#[derive(Clone)]
pub struct Author {
    #[column(primary_key)]
    pub id: uuid::Uuid,
    #[column(unique)]
    pub name: String,
    #[column(name = "is_active")]
    #[index(name = "authors_active_joined")]
    pub active: bool,
    #[column]
    #[index(name = "authors_active_joined")]
    pub joined_at: DateTime<Utc>,
    #[column]
    pub bio: Option<String>,
}
