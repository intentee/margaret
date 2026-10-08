use ephemeral_postgres::cluster::Cluster;
use ephemeral_postgres::cluster_params::ClusterParams;

use margaret_database::database::Database;

use crate::postgres_image::postgres_image;

pub struct StartedDatabase {
    pub database: Database,
    pub ephemeral: ephemeral_postgres::database::Database,
}

impl StartedDatabase {
    /// # Panics
    ///
    /// Panics when the Postgres cluster, its database or the connection to it cannot be prepared.
    pub async fn start() -> Self {
        let ephemeral = Cluster::start(ClusterParams::new(postgres_image()))
            .await
            .expect("the Postgres 18 cluster starts")
            .create_database()
            .await
            .expect("an ephemeral database is created");
        let database = Database::connect(
            ephemeral
                .database_url()
                .parse()
                .expect("the ephemeral database url is a postgres url"),
        )
        .await
        .expect("the ephemeral database accepts connections");

        Self {
            database,
            ephemeral,
        }
    }
}
