use margaret_jwt_verification::jwt_rejection::JwtRejection;

use crate::session::Session;

pub(crate) enum SessionVerification {
    KeysAwaited,
    Rejected(JwtRejection),
    Verified(Session),
}
