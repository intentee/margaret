use serde::Deserialize;

#[derive(Deserialize)]
pub struct CallerClaims {
    pub sub: String,
}
