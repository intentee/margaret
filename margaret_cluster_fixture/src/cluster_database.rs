use margaret::framework::macros::postgres_database;

#[postgres_database(url_from = "MARGARET_CLUSTER_DATABASE_URL")]
pub struct ClusterDatabase;
