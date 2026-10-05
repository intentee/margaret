use sqlx::FromRow;

#[derive(FromRow)]
pub(crate) struct PresentedCodeRow {
    pub(crate) grant_document: String,
    pub(crate) spent: bool,
}
