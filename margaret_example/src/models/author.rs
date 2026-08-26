use chrono::DateTime;
use chrono::Utc;

use margaret::framework::macros::model;

#[model(table = "authors")]
#[index(name = "authors_active_joined", columns = [is_active, joined_at])]
#[derive(Clone)]
pub struct Author {
    #[column(primary_key)]
    pub id: uuid::Uuid,
    #[column(unique)]
    pub name: String,
    #[column(name = "is_active")]
    pub active: bool,
    #[column]
    pub joined_at: DateTime<Utc>,
    #[column]
    pub bio: Option<String>,
    #[column]
    pub reputation: f32,
}
