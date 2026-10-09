use std::sync::Arc;

use tokio_postgres::types::ToSql;

#[derive(Clone, Debug)]
pub struct SqlParameter {
    pub value: Arc<dyn ToSql + Send + Sync>,
}

impl SqlParameter {
    #[must_use]
    pub fn new(value: impl ToSql + Send + Sync + 'static) -> Self {
        Self {
            value: Arc::new(value),
        }
    }
}
