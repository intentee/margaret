use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuditClaims {
    pub scope: String,
}
