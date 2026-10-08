use std::future::ready;

use deadpool_postgres::Manager;
use deadpool_postgres::ManagerConfig;
use deadpool_postgres::Object;
use deadpool_postgres::Pool;
use futures_util::TryFutureExt as _;
use tokio_postgres::NoTls;

use crate::database_error::DatabaseError;
use crate::database_url::DatabaseUrl;

pub struct Database {
    pool: Pool,
}

impl Database {
    /// # Errors
    ///
    /// Returns `DatabaseError::PoolBuild` when the pool cannot be built, and
    /// `DatabaseError::Unavailable` when no connection to the database can be established.
    pub async fn connect(DatabaseUrl { config }: DatabaseUrl) -> Result<Self, DatabaseError> {
        ready(
            Pool::builder(Manager::from_config(
                config,
                NoTls,
                ManagerConfig::default(),
            ))
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
    pub async fn client(&self) -> Result<Object, DatabaseError> {
        self.pool.get().await.map_err(DatabaseError::Unavailable)
    }
}

#[cfg(test)]
mod tests {
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
            )
            .await,
            Err(DatabaseError::Unavailable(_))
        ));
    }
}
