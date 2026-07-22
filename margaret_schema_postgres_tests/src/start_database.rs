use ephemeral_postgres::cluster::Cluster;
use ephemeral_postgres::cluster_params::ClusterParams;
use ephemeral_postgres::database::Database;
use ephemeral_postgres::postgres_image::PostgresImage;

pub async fn start_database() -> Database {
    let image = PostgresImage::new(
        "postgres",
        "18@sha256:3a82e1f56c8f0f5616a11103ac3d47e632c3938698946a7ad26da0df1334744a",
    );

    let cluster = Cluster::start(ClusterParams::new(image))
        .await
        .expect("the Postgres 18 cluster starts");

    cluster
        .create_database()
        .await
        .expect("an ephemeral database is created")
}
