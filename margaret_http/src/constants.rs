use std::time::Duration;

pub(crate) const CLIENT_BODY_TIMEOUT: Duration = Duration::from_mins(1);

pub(crate) const CLIENT_HEAD_TIMEOUT: Duration = Duration::from_secs(30);

pub(crate) const CLIENT_SEND_TIMEOUT: Duration = Duration::from_mins(1);
