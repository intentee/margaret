use serde::Deserialize;

#[derive(Deserialize)]
pub struct ExternalClaims {
    pub sub: String,
}
