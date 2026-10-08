use std::sync::Arc;

use margaret::framework::database::database::Database;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::tokio_postgres::Row;

use crate::models::message::Message;
use crate::stores::cluster_store_error::ClusterStoreError;
use crate::system_clock::SystemClock;

const LIST_MESSAGES: &str = "SELECT id, body, posted_at FROM messages ORDER BY posted_at, id";

const POST_MESSAGE: &str =
    "INSERT INTO messages (body, posted_at) VALUES ($1, $2) RETURNING id, body, posted_at";

fn message_row(row: &Row) -> Result<Message, ClusterStoreError> {
    Ok(Message {
        id: row
            .try_get("id")
            .map_err(ClusterStoreError::MalformedMessageRow)?,
        body: row
            .try_get("body")
            .map_err(ClusterStoreError::MalformedMessageRow)?,
        posted_at: row
            .try_get("posted_at")
            .map_err(ClusterStoreError::MalformedMessageRow)?,
    })
}

#[singleton]
pub struct MessageStore {
    clock: Arc<SystemClock>,
    database: Arc<Database>,
}

impl MessageStore {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(clock: Arc<SystemClock>, database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { clock, database })
    }

    /// # Errors
    ///
    /// Returns `ClusterStoreError` when the messages cannot be read.
    pub async fn list(&self) -> Result<Vec<Message>, ClusterStoreError> {
        self.database
            .client()
            .await
            .map_err(ClusterStoreError::Unavailable)?
            .query(LIST_MESSAGES, &[])
            .await
            .map_err(ClusterStoreError::ListMessages)?
            .iter()
            .map(message_row)
            .collect()
    }

    /// # Errors
    ///
    /// Returns `ClusterStoreError` when the message cannot be stored.
    pub async fn post(&self, body: &str) -> Result<Message, ClusterStoreError> {
        message_row(
            &self
                .database
                .client()
                .await
                .map_err(ClusterStoreError::Unavailable)?
                .query_one(POST_MESSAGE, &[&body, &self.clock.now()])
                .await
                .map_err(ClusterStoreError::PostMessage)?,
        )
    }
}
