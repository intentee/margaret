use chrono::DateTime;
use chrono::Utc;
use futures_util::TryStreamExt as _;
use serde::Serialize;
use uuid::Uuid;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::macros::model;

use crate::stream_batch::STREAM_BATCH;

#[model(table = "notes")]
#[derive(Serialize)]
pub struct Note {
    #[column(primary_key)]
    pub id: Uuid,
    #[column]
    pub body: String,
    #[column]
    #[index]
    pub expires_at: Option<DateTime<Utc>>,
}

impl Note {
    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the notes cannot be read.
    pub async fn in_id_order(database: &Database) -> Result<Vec<Self>, ActiveRecordError> {
        Self::query()
            .id
            .ascending()
            .stream::<STREAM_BATCH, _>(database)
            .try_collect()
            .await
    }
}
