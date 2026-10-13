use std::time::Duration;

pub const ISSUER_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

pub const ISSUER_RESPONSE_MAX_BYTES: usize = 1_048_576;
