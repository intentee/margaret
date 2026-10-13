use std::time::Duration;

pub const AUTHORIZATION_CODE_LIFETIME: Duration = Duration::from_mins(10);

pub const PENDING_AUTHORIZATION_LIFETIME: Duration = Duration::from_mins(10);

pub const REFRESH_FAMILY_LIFETIME: Duration = Duration::from_hours(72);
