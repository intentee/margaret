use std::env;

use ephemeral_postgres::cluster::Cluster;
use ephemeral_postgres::cluster_params::ClusterParams;
use ephemeral_postgres::postgres_image::PostgresImage;
use sqlx::query_scalar;

use margaret_postgres_pool::pg_pool::PgPool;
use margaret_postgres_pool::postgres_connection_uri::PostgresConnectionUri;

#[tokio::test]
async fn connect_opens_a_pool_that_runs_queries() {
    let name = env::var("POSTGRES_IMAGE_NAME")
        .expect("POSTGRES_IMAGE_NAME is not set; run the tests via `make coverage`");
    let tag = env::var("POSTGRES_IMAGE_TAG")
        .expect("POSTGRES_IMAGE_TAG is not set; run the tests via `make coverage`");
    let database = {
        let cluster = Cluster::start(ClusterParams::new(PostgresImage::new(name, tag)))
            .await
            .expect("the Postgres cluster starts");

        cluster
            .create_database()
            .await
            .expect("an ephemeral database is created")
    };
    let uri: PostgresConnectionUri = database
        .database_url()
        .parse()
        .expect("the ephemeral database URL is a valid connection URI");
    let pool = PgPool::connect(uri, 5);

    let answer: i32 = query_scalar("SELECT 1")
        .fetch_one(&*pool)
        .await
        .expect("the lazy pool opens a connection and runs the query");

    assert_eq!(answer, 1);

    pool.close().await;
}
