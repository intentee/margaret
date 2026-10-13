use std::time::Duration;

pub(crate) const FIRST_UNPRIVILEGED_PORT: u16 = 1024;

pub const READINESS_POLL_INTERVAL: Duration = Duration::from_millis(50);
