use std::str::FromStr;

use sqlx::AssertSqlSafe;
use sqlx::raw_sql;
use uuid::Uuid;

use margaret_model::render_postgres::render_postgres;
use margaret_model::schema::Schema;
use margaret_provider_state_postgres::provider_state_tables::provider_state_tables;
use margaret_provider_state_storage::decided_authorization::DecidedAuthorization;
use margaret_provider_state_storage::pending_decision::PendingDecision;
use margaret_provider_state_storage::pending_verdict::PendingVerdict;
use margaret_provider_state_storage_selection::provider_state_storage_uri::ProviderStateStorageUri;
use margaret_provider_state_storage_selection::resolve_provider_state_storage::resolve_provider_state_storage;
use margaret_schema_postgres_tests::start_database::start_database;

#[tokio::test]
async fn postgres_resolve_selects_postgres_storage() {
    let database = start_database().await;

    raw_sql(AssertSqlSafe(render_postgres(&Schema {
        tables: provider_state_tables(),
    })))
    .execute(database.pool())
    .await
    .expect("the provider state tables apply to Postgres");

    let state = resolve_provider_state_storage(
        ProviderStateStorageUri::from_str(database.database_url())
            .expect("the database url selects the postgres storage"),
    );

    assert_eq!(
        state
            .decide_pending_authorization(
                Uuid::new_v4(),
                PendingDecision {
                    subject: Uuid::new_v4(),
                    verdict: PendingVerdict::Denied,
                },
            )
            .await
            .expect("the postgres storage decides"),
        DecidedAuthorization::Unknown
    );
}
