#[rustfmt::skip]
#[path = "../margaret/mod.rs"]
pub mod margaret;

pub mod consumed_sessions;
pub mod cookie_domain;
pub mod issued_sessions;
pub mod resolved_session;
pub mod session;
mod session_cookie_jar;
pub mod session_cookies;
pub mod session_endpoint;
pub mod session_lifetime_secs;
pub mod session_record;
mod session_refresh;
pub mod session_refresh_answer;
pub mod session_refresh_endpoint;
mod session_refresh_request;
pub mod session_refresh_url;
pub mod session_resolution;
pub mod session_sign_out_endpoint;
pub mod session_unavailability;
mod session_verification;
pub mod sessions_error;
pub mod started_session;
