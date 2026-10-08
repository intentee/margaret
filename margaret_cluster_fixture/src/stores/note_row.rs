use margaret::framework::tokio_postgres::Row;

use crate::models::note::Note;
use crate::stores::cluster_store_error::ClusterStoreError;

pub(crate) fn note_row(row: &Row) -> Result<Note, ClusterStoreError> {
    Ok(Note {
        id: row
            .try_get("id")
            .map_err(ClusterStoreError::MalformedNoteRow)?,
        body: row
            .try_get("body")
            .map_err(ClusterStoreError::MalformedNoteRow)?,
        expires_at: row
            .try_get("expires_at")
            .map_err(ClusterStoreError::MalformedNoteRow)?,
    })
}
