use chrono::DateTime;
use chrono::Utc;
use futures_util::TryStreamExt as _;
use serde::Serialize;
use uuid::Uuid;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::macros::model;
use margaret::framework::model::column_default::ColumnDefault;

use crate::stream_batch::STREAM_BATCH;

#[model(table = "messages")]
#[index(name = "messages_posted", fields = [posted_at, id])]
#[derive(Serialize)]
pub struct Message {
    #[column(primary_key, default = ColumnDefault::UuidV7)]
    pub id: Uuid,
    #[column]
    pub body: String,
    #[column]
    pub posted_at: DateTime<Utc>,
}

impl Message {
    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the messages cannot be read.
    pub async fn in_posting_order(database: &Database) -> Result<Vec<Self>, ActiveRecordError> {
        Self::query()
            .posted_at
            .ascending()
            .stream::<STREAM_BATCH, _>(database)
            .try_collect()
            .await
    }
}
