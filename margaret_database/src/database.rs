use std::future::ready;

use deadpool_postgres::Manager;
use deadpool_postgres::ManagerConfig;
use deadpool_postgres::Object;
use deadpool_postgres::Pool;
use futures_util::TryFutureExt as _;
use tokio_postgres::NoTls;
use tokio_postgres::Row;

use margaret_sql::statement::Statement;

use crate::database_error::DatabaseError;
use crate::database_url::DatabaseUrl;
use crate::executor::Executor;
use crate::executor_seal::ExecutorSeal;
use crate::max_connections::MaxConnections;
use crate::pooled_connection::PooledConnection;
use crate::statement_affected::statement_affected;
use crate::statement_optional_row::statement_optional_row;
use crate::statement_row::statement_row;
use crate::statement_rows::statement_rows;

pub struct Database {
    pool: Pool,
}

impl Database {
    /// # Errors
    ///
    /// Returns `DatabaseError::PoolBuild` when the pool cannot be built, and
    /// `DatabaseError::Unavailable` when no connection to the database can be established.
    pub async fn connect(
        DatabaseUrl { config }: DatabaseUrl,
        MaxConnections { connections }: MaxConnections,
    ) -> Result<Self, DatabaseError> {
        ready(
            Pool::builder(Manager::from_config(
                config,
                NoTls,
                ManagerConfig::default(),
            ))
            .max_size(connections.get())
            .build()
            .map_err(DatabaseError::PoolBuild),
        )
        .and_then(|pool| async move {
            pool.get()
                .await
                .map_err(DatabaseError::Unavailable)
                .map(|_connection| Self { pool })
        })
        .await
    }

    /// # Errors
    ///
    /// Returns `DatabaseError::Unavailable` when no connection to the database can be checked out.
    pub async fn connection(&self) -> Result<PooledConnection, DatabaseError> {
        self.client()
            .await
            .map(|object| PooledConnection { object })
    }

    async fn client(&self) -> Result<Object, DatabaseError> {
        self.pool.get().await.map_err(DatabaseError::Unavailable)
    }
}

impl ExecutorSeal for Database {}

impl Executor for Database {
    async fn affected(&self, statement: &Statement) -> Result<u64, DatabaseError> {
        statement_affected(&self.client().await?, statement).await
    }

    async fn optional_row(&self, statement: &Statement) -> Result<Option<Row>, DatabaseError> {
        statement_optional_row(&self.client().await?, statement).await
    }

    async fn row(&self, statement: &Statement) -> Result<Row, DatabaseError> {
        statement_row(&self.client().await?, statement).await
    }

    async fn rows(&self, statement: &Statement) -> Result<Vec<Row>, DatabaseError> {
        statement_rows(&self.client().await?, statement).await
    }
}

#[cfg(test)]
mod tests {
    use deadpool_postgres::PoolError;
    use tokio::net::TcpListener;

    use super::Database;
    use crate::database_error::DatabaseError;

    #[tokio::test]
    async fn reports_an_unreachable_database() {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("a free port is reserved");
        let port = listener
            .local_addr()
            .expect("the reserved port is known")
            .port();

        drop(listener);

        assert!(matches!(
            Database::connect(
                format!("postgresql://margaret@127.0.0.1:{port}/blog")
                    .parse()
                    .expect("the url is a postgres url"),
                "1".parse().expect("one connection is a pool size"),
            )
            .await,
            Err(DatabaseError::Unavailable(PoolError::Backend(source))) if source.as_db_error().is_none()
        ));
    }
}
