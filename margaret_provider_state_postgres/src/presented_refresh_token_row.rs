use sqlx::FromRow;

#[derive(FromRow)]
pub(crate) struct PresentedRefreshTokenRow {
    pub(crate) grant_document: String,
    pub(crate) superseded: bool,
}
