use std::sync::Arc;

use ephemeral_postgres::database::Database;

use margaret::framework::postgres_pool::pg_pool::PgPool;
use margaret::framework::postgres_pool::postgres_connection_uri::PostgresConnectionUri;
use margaret::framework::postgres_pool::postgres_max_connections::PostgresMaxConnections;
use margaret_example::stores::article_store::ArticleStore;
use margaret_example::system_clock::SystemClock;

#[must_use]
pub fn build_article_store(database: &Database) -> Arc<ArticleStore> {
    let uri: PostgresConnectionUri = database
        .database_url()
        .parse()
        .expect("the ephemeral database URL is a valid connection URI");
    let max_connections: PostgresMaxConnections =
        "5".parse().expect("5 is a valid max connections");
    let pool = Arc::new(PgPool::connect(uri, max_connections));
    let clock = Arc::new(SystemClock::create());

    Arc::new(ArticleStore::create(clock, pool))
}
