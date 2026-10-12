use serde::Deserialize;

#[derive(Deserialize)]
pub struct AttachmentClaims {
    pub sub: String,
}
