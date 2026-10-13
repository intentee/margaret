use margaret_sessions::session::Session;

use crate::authenticated_end_user::AuthenticatedEndUser;

pub(crate) fn end_user_of(
    Session {
        authenticated_at,
        subject,
        ..
    }: Session,
) -> AuthenticatedEndUser {
    AuthenticatedEndUser {
        authenticated_at,
        subject,
    }
}
