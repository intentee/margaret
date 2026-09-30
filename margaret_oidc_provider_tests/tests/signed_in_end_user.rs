use chrono::Utc;

use margaret_oidc_provider::authenticated_end_user::AuthenticatedEndUser;

use crate::end_user_subject::END_USER_SUBJECT;

pub fn signed_in_end_user() -> AuthenticatedEndUser {
    AuthenticatedEndUser {
        authenticated_at: Utc::now(),
        subject: END_USER_SUBJECT,
    }
}
