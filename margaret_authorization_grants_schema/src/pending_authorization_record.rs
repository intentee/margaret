use uuid::Uuid;

use margaret_macros::model;

#[model(table = "pending_authorizations")]
pub struct PendingAuthorizationRecord {
    #[column(primary_key)]
    pub id: Uuid,
    #[column]
    #[index]
    pub expires_at: i64,
    #[column]
    pub grant: String,
    #[column]
    pub state: Option<String>,
}
