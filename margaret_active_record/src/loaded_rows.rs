use tokio_postgres::Row;

pub struct LoadedRows<'rows> {
    pub(crate) rows: &'rows [Row],
}
