use margaret_postgres_pool::pg_pool::PgPool;
use margaret_postgres_pool::postgres_connection_uri::PostgresConnectionUri;
use margaret_postgres_pool::postgres_max_connections::PostgresMaxConnections;
use margaret_postgres_pool::postgres_pool_error::PostgresPoolError;

#[tokio::test]
async fn ensure_reachable_reports_an_unreachable_database() {
    let uri: PostgresConnectionUri =
        "postgres://postgres:postgres@127.0.0.1/postgres?host=/nonexistent-socket"
            .parse()
            .expect("a valid postgres URL parses");
    let max_connections: PostgresMaxConnections = "1".parse().expect("1 is a valid max connections");
    let pool = PgPool::connect(uri, max_connections);

    let error = pool
        .ensure_reachable()
        .await
        .expect_err("an unreachable database is reported");

    assert!(matches!(error, PostgresPoolError::Unreachable { .. }));
}
