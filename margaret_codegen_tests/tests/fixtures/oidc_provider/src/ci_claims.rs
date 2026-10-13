use serde::Deserialize;

#[derive(Deserialize)]
pub struct CiClaims {
    pub repository: String,
}
