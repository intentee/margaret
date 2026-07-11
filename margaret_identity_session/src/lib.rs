pub mod access_token_claims;
pub mod access_token_claims_signed;
pub mod identity_session_error;
pub mod refresh_token_claims;
pub mod refresh_token_claims_signed;
pub mod session_cookie_manager;

pub const COOKIE_NAME_ACCESS_TOKEN: &str = "access_token";
pub const COOKIE_NAME_REFRESH_TOKEN: &str = "refresh_token";

pub const ACCESS_TOKEN_LIFETIME_SECS: i64 = 15 * 60;
pub const REFRESH_TOKEN_LIFETIME_SECS: i64 = 60 * 60 * 24 * 3;
