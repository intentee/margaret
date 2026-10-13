use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use margaret::framework::macros::model;
use margaret::framework::model::column_default::ColumnDefault;

#[model(table = "authors")]
#[index(name = "authors_active_joined", fields = [active, joined_at])]
#[derive(Clone, Debug, PartialEq)]
pub struct Author {
    #[column(primary_key, default = ColumnDefault::UuidV7)]
    pub id: Uuid,
    #[column(unique)]
    pub name: String,
    #[column(name = "is_active")]
    pub active: bool,
    #[column]
    pub joined_at: DateTime<Utc>,
    #[column]
    pub bio: Option<String>,
}
