use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthenticatedEndUser {
    pub authenticated_at: DateTime<Utc>,
    pub subject: Uuid,
}
