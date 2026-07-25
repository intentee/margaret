use std::ops::Deref;

use sqlx::postgres::PgPoolOptions;

use crate::postgres_connection_uri::PostgresConnectionUri;

#[derive(Clone)]
pub struct PgPool {
    pool: sqlx::PgPool,
}

impl PgPool {
    #[must_use]
    pub fn connect(postgres_url: PostgresConnectionUri, max_connections: u32) -> Self {
        Self {
            pool: PgPoolOptions::new()
                .max_connections(max_connections)
                .connect_lazy_with(postgres_url.into_connect_options()),
        }
    }
}

impl Deref for PgPool {
    type Target = sqlx::PgPool;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

#[cfg(test)]
mod tests {
    use super::PgPool;
    use crate::postgres_connection_uri::PostgresConnectionUri;

    #[tokio::test]
    async fn connect_builds_a_lazy_pool_with_the_requested_max_connections() {
        let uri: PostgresConnectionUri = "postgres://user:secret@localhost:5432/app"
            .parse()
            .expect("a valid postgres URL parses");

        let pool = PgPool::connect(uri, 7);

        assert_eq!(pool.options().get_max_connections(), 7);
        assert_eq!(pool.size(), 0);
    }
}
