use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use margaret_macros::model;

#[model(table = "refresh_families")]
pub struct RefreshFamilyRecord {
    #[column(primary_key)]
    pub family: Uuid,
    #[column]
    pub auth_time: DateTime<Utc>,
    #[column]
    pub client_id: String,
    #[column]
    #[index]
    pub expires_at: i64,
    #[column]
    pub scopes: String,
    #[column]
    pub subject: Uuid,
}
