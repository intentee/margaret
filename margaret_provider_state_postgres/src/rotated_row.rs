use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub(crate) struct RotatedRow {
    pub(crate) client_matches: bool,
    pub(crate) family: Uuid,
    pub(crate) grant_document: Option<String>,
    pub(crate) scope_within: bool,
    pub(crate) superseded: bool,
}
