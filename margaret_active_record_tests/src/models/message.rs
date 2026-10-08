use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use margaret::framework::macros::model;

#[model(table = "messages")]
#[index(name = "messages_posted", fields = [posted_at, id])]
#[derive(Clone, Debug, PartialEq)]
pub struct Message {
    #[column(primary_key)]
    pub id: Uuid,
    #[column]
    pub body: String,
    #[column]
    pub posted_at: DateTime<Utc>,
}
