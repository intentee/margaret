use std::future::Future;

use tokio_postgres::Row;

use margaret_sql::statement::Statement;

use crate::database_error::DatabaseError;
use crate::executor_seal::ExecutorSeal;

pub trait Executor: ExecutorSeal + Sync {
    fn affected(
        &self,
        statement: &Statement,
    ) -> impl Future<Output = Result<u64, DatabaseError>> + Send;

    fn optional_row(
        &self,
        statement: &Statement,
    ) -> impl Future<Output = Result<Option<Row>, DatabaseError>> + Send;

    fn row(&self, statement: &Statement)
    -> impl Future<Output = Result<Row, DatabaseError>> + Send;

    fn rows(
        &self,
        statement: &Statement,
    ) -> impl Future<Output = Result<Vec<Row>, DatabaseError>> + Send;
}
