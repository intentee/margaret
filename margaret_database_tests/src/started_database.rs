use std::num::NonZeroUsize;
use std::sync::Arc;

use url::Url;
use uuid::Uuid;

use margaret_database::database::Database;
use margaret_database::max_connections::MaxConnections;
use margaret_model::render_postgres::render_postgres;
use margaret_model::schema::Schema;
use margaret_sql_identifier::quote_identifier::quote_identifier;

use crate::connected_client::connected_client;
use crate::database_administration::DatabaseAdministration;
use crate::racing_instances::RACING_INSTANCES;
use crate::test_postgres_url::test_postgres_url;

fn racing_pool_size() -> MaxConnections {
    MaxConnections {
        connections: NonZeroUsize::new(RACING_INSTANCES)
            .expect("the racing instances of a test are at least one"),
    }
}

async fn connected_pool(database_url: &Url, max_connections: MaxConnections) -> Database {
    Database::connect(
        database_url
            .as_str()
            .parse()
            .expect("the test database url is a postgres url"),
        max_connections,
    )
    .await
    .expect("the test database accepts connections")
}

pub struct StartedDatabase {
    pub administration: DatabaseAdministration,
    pub database: Arc<Database>,
    pub database_url: Url,
}

impl StartedDatabase {
    /// # Panics
    ///
    /// Panics when the role, the database or the connection to it cannot be prepared on the shared
    /// test cluster.
    pub async fn start() -> Self {
        let cluster_url = test_postgres_url();
        let name = format!("test_{}", Uuid::new_v4().simple());
        let administration = connected_client(&cluster_url).await;

        administration
            .batch_execute(&format!("CREATE ROLE {} LOGIN", quote_identifier(&name)))
            .await
            .expect("the role of the test is created");
        administration
            .batch_execute(&format!(
                "CREATE DATABASE {} OWNER {} STRATEGY FILE_COPY",
                quote_identifier(&name),
                quote_identifier(&name)
            ))
            .await
            .expect("the database of the test is created");

        let mut administration_url = cluster_url.clone();
        let mut database_url = cluster_url.clone();

        administration_url.set_path(&name);
        database_url
            .set_username(&name)
            .expect("the shared test cluster url carries a username");
        database_url.set_path(&name);

        let database = connected_pool(&database_url, racing_pool_size()).await;

        Self {
            administration: DatabaseAdministration {
                cluster_url,
                database_url: administration_url,
                name,
            },
            database: Arc::new(database),
            database_url,
        }
    }

    pub async fn with_schema(schema: &Schema) -> Self {
        let started = Self::start().await;

        started.execute(&render_postgres(schema)).await;

        started
    }

    /// # Panics
    ///
    /// Panics when the role of the test cannot execute the setup statements.
    pub async fn execute(&self, statements: &str) {
        connected_client(&self.database_url)
            .await
            .batch_execute(statements)
            .await
            .expect("the role of the test executes the setup statements");
    }

    /// # Panics
    ///
    /// Panics when the test database refuses another pool of connections.
    pub async fn separate_pool(&self) -> Arc<Database> {
        self.separate_pool_of(racing_pool_size()).await
    }

    /// # Panics
    ///
    /// Panics when the test database refuses another pool of connections.
    pub async fn separate_pool_of(&self, max_connections: MaxConnections) -> Arc<Database> {
        Arc::new(connected_pool(&self.database_url, max_connections).await)
    }
}
