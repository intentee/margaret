use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

use margaret::framework::macros::model;

#[model(table = "notes")]
#[derive(Serialize)]
pub struct Note {
    #[column(primary_key)]
    pub id: Uuid,
    #[column]
    pub body: String,
    #[column]
    #[index]
    pub expires_at: Option<DateTime<Utc>>,
}
