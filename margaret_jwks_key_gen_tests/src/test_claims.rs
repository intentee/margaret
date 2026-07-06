use serde::Deserialize;
use serde::Serialize;

pub const ALREADY_EXPIRED_EXPIRY: usize = 1;
pub const FAR_FUTURE_EXPIRY: usize = 9_999_999_999;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TestClaims {
    pub exp: usize,
    pub sub: String,
}
