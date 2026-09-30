use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use margaret::framework::oidc_provider::authenticated_end_user::AuthenticatedEndUser;

#[derive(Clone)]
pub struct User {
    pub authenticated_at: DateTime<Utc>,
    pub id: Uuid,
    pub name: String,
}

impl User {
    #[must_use]
    pub fn end_user(&self) -> AuthenticatedEndUser {
        AuthenticatedEndUser {
            authenticated_at: self.authenticated_at,
            subject: self.id,
        }
    }
}
