use uuid::Uuid;

use margaret_macros::model;

use crate::refresh_family_record::RefreshFamilyRecord;

#[model(table = "refresh_tokens")]
#[foreign_key(columns = [family], references = RefreshFamilyRecord)]
pub struct RefreshTokenRecord {
    #[column(primary_key, byte_length = 32)]
    pub token: Vec<u8>,
    #[column]
    #[index]
    pub family: Uuid,
    #[column]
    pub current: bool,
}
