use tokio::time::MissedTickBehavior;
use tokio::time::interval;
use tokio_postgres::Client;
use url::Url;

use margaret_sql_identifier::qualified_table::qualified_table;
use margaret_sql_identifier::quote_identifier::quote_identifier;
use margaret_sql_identifier::table_namespace::TableNamespace;

use crate::connected_client::connected_client;
use crate::lock_waiters_poll_interval::LOCK_WAITERS_POLL_INTERVAL;
use crate::table_privilege::TablePrivilege;

pub struct DatabaseAdministration {
    pub(crate) cluster_url: Url,
    pub(crate) database_url: Url,
    pub(crate) name: String,
}

impl DatabaseAdministration {
    /// # Panics
    ///
    /// Panics when the shared test cluster cannot report the backends waiting on locks.
    pub async fn await_lock_waiters(&self, count: i64) {
        let client = connected_client(&self.cluster_url).await;
        let mut polls = interval(LOCK_WAITERS_POLL_INTERVAL);

        polls.set_missed_tick_behavior(MissedTickBehavior::Delay);

        while client
            .query_one(
                "SELECT count(*) FROM pg_stat_activity WHERE datname = $1 AND wait_event_type = 'Lock'",
                &[&self.name],
            )
            .await
            .expect("the backends waiting on locks are counted")
            .get::<_, i64>(0)
            < count
        {
            polls.tick().await;
        }
    }

    pub async fn client(&self) -> Client {
        connected_client(&self.database_url).await
    }

    /// # Panics
    ///
    /// Panics when the setup statements cannot be executed on the test database.
    pub async fn execute(&self, statements: &str) {
        self.client()
            .await
            .batch_execute(statements)
            .await
            .expect("the setup statements are executed on the test database");
    }

    /// # Panics
    ///
    /// Panics when the rows of the table cannot be hidden from the role of the test.
    pub async fn hide_rows(&self, namespace: TableNamespace, table: &str, visible_when: &str) {
        let table = qualified_table(namespace, table);

        self.execute(&format!(
            "ALTER TABLE {table} ENABLE ROW LEVEL SECURITY; \
             ALTER TABLE {table} FORCE ROW LEVEL SECURITY; \
             CREATE POLICY insertions ON {table} FOR INSERT WITH CHECK (true); \
             CREATE POLICY visible_rows ON {table} FOR SELECT USING ({visible_when})"
        ))
        .await;
    }

    /// # Panics
    ///
    /// Panics when the shared test cluster cannot stop admitting connections to the test database.
    pub async fn make_unreachable(&self) {
        let client = connected_client(&self.cluster_url).await;

        client
            .batch_execute(&format!(
                "ALTER DATABASE {} ALLOW_CONNECTIONS false",
                quote_identifier(&self.name)
            ))
            .await
            .expect("the test database stops admitting connections");
        client
            .execute(
                "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = $1",
                &[&self.name],
            )
            .await
            .expect("the connections to the test database are terminated");
    }

    /// # Panics
    ///
    /// Panics when the privilege cannot be revoked from the role of the test.
    pub async fn revoke(&self, privilege: TablePrivilege, namespace: TableNamespace, table: &str) {
        self.execute(&format!(
            "REVOKE {} ON {} FROM {}",
            privilege.keyword(),
            qualified_table(namespace, table),
            quote_identifier(&self.name)
        ))
        .await;
    }
}
