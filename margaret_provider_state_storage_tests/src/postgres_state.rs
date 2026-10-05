use std::sync::Arc;

use ephemeral_postgres::database::Database;
use sqlx::AssertSqlSafe;
use sqlx::raw_sql;

use margaret_model::render_postgres::render_postgres;
use margaret_model::schema::Schema;
use margaret_provider_state_postgres::postgres_provider_state::PostgresProviderState;
use margaret_provider_state_postgres::provider_state_tables::provider_state_tables;
use margaret_schema_postgres_tests::start_database::start_database;

fn quoted_table_names() -> String {
    provider_state_tables()
        .iter()
        .map(|table| format!("\"{}\"", table.name))
        .collect::<Vec<String>>()
        .join(", ")
}

pub struct PostgresState {
    pub database: Database,
    pub state: Arc<PostgresProviderState>,
}

impl PostgresState {
    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    pub async fn with_tables() -> Self {
        let database = start_database().await;

        raw_sql(AssertSqlSafe(render_postgres(&Schema {
            tables: provider_state_tables(),
        })))
        .execute(database.pool())
        .await
        .expect("the provider state tables apply to Postgres");

        let state = Arc::new(PostgresProviderState::create(database.pool().clone()));

        Self { database, state }
    }

    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    pub async fn dropped(&self) {
        raw_sql(AssertSqlSafe(format!(
            "DROP TABLE {} CASCADE",
            quoted_table_names()
        )))
        .execute(self.database.pool())
        .await
        .expect("the provider state tables are dropped");
    }
}
