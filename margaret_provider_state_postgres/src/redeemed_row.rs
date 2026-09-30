use sqlx::FromRow;

#[derive(FromRow)]
pub(crate) struct RedeemedRow {
    pub(crate) admitted: bool,
    pub(crate) grant_document: String,
    pub(crate) replayed: bool,
}
