use margaret_http::cookie_changes::CookieChanges;

use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

pub struct SessionUserInference<User> {
    pub cookie_changes: CookieChanges,
    pub outcome: AuthenticatedUserOutcome<User>,
}
