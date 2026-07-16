use std::time::Duration;

use margaret_identity_session::refresh_token_lifetime_secs::REFRESH_TOKEN_LIFETIME_SECS;

pub const ROTATION_INTERVAL: Duration = Duration::from_secs(REFRESH_TOKEN_LIFETIME_SECS as u64);
