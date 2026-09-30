use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

pub struct UserSession {
    pub authenticated_at: DateTime<Utc>,
    pub user_id: Uuid,
}
