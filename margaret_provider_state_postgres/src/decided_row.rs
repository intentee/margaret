use sqlx::FromRow;

#[derive(FromRow)]
pub(crate) struct DecidedRow {
    pub(crate) pending_document: String,
    pub(crate) subject_matches: bool,
}
