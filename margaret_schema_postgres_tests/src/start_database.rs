use ephemeral_postgres::cluster::Cluster;
use ephemeral_postgres::cluster_params::ClusterParams;
use ephemeral_postgres::database::Database;

use crate::postgres_image::postgres_image;

pub async fn start_database() -> Database {
    let cluster = Cluster::start(ClusterParams::new(postgres_image()))
        .await
        .expect("the Postgres 18 cluster starts");

    cluster
        .create_database()
        .await
        .expect("an ephemeral database is created")
}
