use tokio_postgres::Row;

use margaret_sql::statement::Statement;

use crate::database_error::DatabaseError;
use crate::executor::Executor;
use crate::executor_seal::ExecutorSeal;
use crate::statement_affected::statement_affected;
use crate::statement_optional_row::statement_optional_row;
use crate::statement_row::statement_row;
use crate::statement_rows::statement_rows;

pub struct Transaction<'connection> {
    pub(crate) inner: deadpool_postgres::Transaction<'connection>,
}

impl Transaction<'_> {
    /// # Errors
    ///
    /// Returns `DatabaseError::Commit` when the database does not commit the transaction.
    pub async fn commit(self) -> Result<(), DatabaseError> {
        self.inner.commit().await.map_err(DatabaseError::Commit)
    }

    /// # Errors
    ///
    /// Returns `DatabaseError::Rollback` when the database does not roll the transaction back.
    pub async fn rollback(self) -> Result<(), DatabaseError> {
        self.inner.rollback().await.map_err(DatabaseError::Rollback)
    }
}

impl ExecutorSeal for Transaction<'_> {}

impl Executor for Transaction<'_> {
    async fn affected(&self, statement: &Statement) -> Result<u64, DatabaseError> {
        statement_affected(&self.inner, statement).await
    }

    async fn optional_row(&self, statement: &Statement) -> Result<Option<Row>, DatabaseError> {
        statement_optional_row(&self.inner, statement).await
    }

    async fn row(&self, statement: &Statement) -> Result<Row, DatabaseError> {
        statement_row(&self.inner, statement).await
    }

    async fn rows(&self, statement: &Statement) -> Result<Vec<Row>, DatabaseError> {
        statement_rows(&self.inner, statement).await
    }
}
