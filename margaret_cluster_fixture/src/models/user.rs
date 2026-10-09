use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

#[derive(Clone)]
pub struct User {
    pub authenticated_at: DateTime<Utc>,
    pub id: Uuid,
    pub name: String,
}
