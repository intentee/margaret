use serde::Serialize;

#[derive(Serialize)]
pub struct ProfileClaims {
    pub name: String,
}
