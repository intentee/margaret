use std::time::Duration;

use margaret_http::request_timeout::RequestTimeout;

#[must_use]
pub fn brief_request_timeout() -> RequestTimeout {
    RequestTimeout::new(Duration::from_millis(25))
}
