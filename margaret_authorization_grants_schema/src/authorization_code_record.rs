use uuid::Uuid;

use margaret_macros::model;

#[model(table = "authorization_codes")]
pub struct AuthorizationCodeRecord {
    #[column(primary_key, byte_length = 32)]
    pub code: Vec<u8>,
    #[column]
    #[index]
    pub expires_at: i64,
    #[column]
    pub grant: String,
    #[column]
    pub redeemed_by: Option<Uuid>,
}
