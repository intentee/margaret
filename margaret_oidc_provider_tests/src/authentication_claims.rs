use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthenticationClaims {
    pub auth_time: i64,
}
