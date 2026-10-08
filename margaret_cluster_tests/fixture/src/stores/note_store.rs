use std::sync::Arc;

use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use margaret::framework::database::database::Database;
use margaret::framework::macros::constructor;
use margaret::framework::macros::provides_route_parameter;
use margaret::framework::macros::singleton;
use margaret::framework::route_parameter_binding::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret::framework::route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;

use crate::models::note::Note;
use crate::stores::cluster_store_error::ClusterStoreError;
use crate::stores::note_row::note_row;

const DELETE_NOTE: &str = "DELETE FROM notes WHERE id = $1";

const FIND_NOTE: &str = "SELECT id, body, expires_at FROM notes WHERE id = $1";

const INSERT_NOTE: &str =
    "INSERT INTO notes (body, expires_at) VALUES ($1, $2) RETURNING id, body, expires_at";

const LIST_NOTES: &str = "SELECT id, body, expires_at FROM notes ORDER BY id";

const SWEEP_NOTES: &str = "DELETE FROM notes WHERE expires_at <= $1";

const UPDATE_NOTE: &str = "UPDATE notes SET body = $2 WHERE id = $1";

#[singleton]
#[provides_route_parameter]
pub struct NoteStore {
    database: Arc<Database>,
}

impl NoteStore {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { database })
    }

    /// # Errors
    ///
    /// Returns `ClusterStoreError` when the note cannot be deleted.
    pub async fn delete(&self, id: Uuid) -> Result<(), ClusterStoreError> {
        self.database
            .client()
            .await
            .map_err(ClusterStoreError::Unavailable)?
            .execute(DELETE_NOTE, &[&id])
            .await
            .map_err(ClusterStoreError::DeleteNote)
            .map(|_deleted| ())
    }

    /// # Errors
    ///
    /// Returns `ClusterStoreError` when the note cannot be read.
    pub async fn find(&self, id: Uuid) -> Result<Option<Note>, ClusterStoreError> {
        self.database
            .client()
            .await
            .map_err(ClusterStoreError::Unavailable)?
            .query_opt(FIND_NOTE, &[&id])
            .await
            .map_err(ClusterStoreError::FindNote)?
            .as_ref()
            .map(note_row)
            .transpose()
    }

    /// # Errors
    ///
    /// Returns `ClusterStoreError` when the note cannot be stored.
    pub async fn insert(
        &self,
        body: &str,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<Note, ClusterStoreError> {
        note_row(
            &self
                .database
                .client()
                .await
                .map_err(ClusterStoreError::Unavailable)?
                .query_one(INSERT_NOTE, &[&body, &expires_at])
                .await
                .map_err(ClusterStoreError::InsertNote)?,
        )
    }

    /// # Errors
    ///
    /// Returns `ClusterStoreError` when the notes cannot be read.
    pub async fn list(&self) -> Result<Vec<Note>, ClusterStoreError> {
        self.database
            .client()
            .await
            .map_err(ClusterStoreError::Unavailable)?
            .query(LIST_NOTES, &[])
            .await
            .map_err(ClusterStoreError::ListNotes)?
            .iter()
            .map(note_row)
            .collect()
    }

    /// # Errors
    ///
    /// Returns `ClusterStoreError` when the expired notes cannot be deleted.
    pub async fn sweep(&self, now: DateTime<Utc>) -> Result<u64, ClusterStoreError> {
        self.database
            .client()
            .await
            .map_err(ClusterStoreError::Unavailable)?
            .execute(SWEEP_NOTES, &[&now])
            .await
            .map_err(ClusterStoreError::SweepNotes)
    }

    /// # Errors
    ///
    /// Returns `ClusterStoreError` when the note cannot be updated.
    pub async fn update(&self, id: Uuid, body: &str) -> Result<(), ClusterStoreError> {
        self.database
            .client()
            .await
            .map_err(ClusterStoreError::Unavailable)?
            .execute(UPDATE_NOTE, &[&id, &body])
            .await
            .map_err(ClusterStoreError::UpdateNote)
            .map(|_updated| ())
    }
}

#[async_trait]
impl HttpRouteParameterBinder for NoteStore {
    type Model = Note;

    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Note>> {
        Ok(match Uuid::parse_str(&value) {
            Ok(id) => match self.find(id).await? {
                Some(note) => RouteParameterBindingOutcome::Bound(note),
                None => RouteParameterBindingOutcome::NotFound,
            },
            Err(_malformed) => RouteParameterBindingOutcome::NotFound,
        })
    }
}
