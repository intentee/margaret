use serde::Deserialize;

#[derive(Deserialize)]
pub struct EmailClaims {
    pub email: String,
}
