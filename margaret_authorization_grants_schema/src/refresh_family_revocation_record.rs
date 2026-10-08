use uuid::Uuid;

use margaret_macros::model;

#[model(table = "refresh_family_revocations")]
pub struct RefreshFamilyRevocationRecord {
    #[column(primary_key)]
    pub family: Uuid,
    #[column]
    #[index]
    pub expires_at: i64,
}
