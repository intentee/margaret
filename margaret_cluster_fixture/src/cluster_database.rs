use margaret::framework::macros::postgres_database;

#[postgres_database(
    url_from = "MARGARET_CLUSTER_DATABASE_URL",
    max_connections_from = "MARGARET_CLUSTER_DATABASE_MAX_CONNECTIONS"
)]
pub struct ClusterDatabase;
