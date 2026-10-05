use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub(crate) struct SettlementRow {
    pub(crate) family: Uuid,
    pub(crate) replayed: bool,
}
