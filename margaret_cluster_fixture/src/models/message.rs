use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

use margaret::framework::macros::model;

#[model(table = "messages")]
#[derive(Serialize)]
pub struct Message {
    #[column(primary_key)]
    pub id: Uuid,
    #[column]
    pub body: String,
    #[column]
    #[index]
    pub posted_at: DateTime<Utc>,
}
