use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PartnerClaims {
    pub reader: Uuid,
    pub verified_reader: bool,
}
