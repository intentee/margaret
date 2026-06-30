use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct AccessTokenClaims {
    pub exp: i64,
    pub iat: i64,
    pub sub: String,
}
