use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TestClaims {
    pub exp: usize,
    pub sub: String,
}
