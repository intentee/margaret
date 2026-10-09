use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use margaret::framework::macros::model;
use margaret::framework::model::column_default::ColumnDefault;

#[model(table = "notes")]
#[derive(Clone, Debug, PartialEq)]
pub struct Note {
    #[column(primary_key, default = ColumnDefault::UuidV7)]
    pub id: Uuid,
    #[column]
    pub body: String,
    #[column]
    #[index]
    pub expires_at: Option<DateTime<Utc>>,
}
