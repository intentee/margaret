use deadpool_postgres::Object;

use crate::database_error::DatabaseError;
use crate::isolation::Isolation;
use crate::transaction::Transaction;

pub struct PooledConnection {
    pub(crate) object: Object,
}

impl PooledConnection {
    /// # Errors
    ///
    /// Returns `DatabaseError::Begin` when the database does not begin the transaction.
    pub async fn transaction(
        &mut self,
        isolation: Isolation,
    ) -> Result<Transaction<'_>, DatabaseError> {
        self.object
            .build_transaction()
            .isolation_level(isolation.level())
            .start()
            .await
            .map(|inner| Transaction { inner })
            .map_err(|source| DatabaseError::Begin { isolation, source })
    }
}
