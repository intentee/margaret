use std::sync::Arc;

use uuid::Uuid;

use margaret::framework::database::database::Database;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use crate::stores::cluster_store_error::ClusterStoreError;

const FIND_UPLOAD: &str = "SELECT content FROM uploads WHERE id = $1";

const INSERT_UPLOAD: &str = "INSERT INTO uploads (content) VALUES ($1) RETURNING id";

#[singleton]
pub struct UploadStore {
    database: Arc<Database>,
}

impl UploadStore {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { database })
    }

    /// # Errors
    ///
    /// Returns `ClusterStoreError` when the upload cannot be read.
    pub async fn find(&self, id: Uuid) -> Result<Option<Vec<u8>>, ClusterStoreError> {
        self.database
            .client()
            .await
            .map_err(ClusterStoreError::Unavailable)?
            .query_opt(FIND_UPLOAD, &[&id])
            .await
            .map_err(ClusterStoreError::FindUpload)?
            .map(|row| {
                row.try_get("content")
                    .map_err(ClusterStoreError::MalformedUploadRow)
            })
            .transpose()
    }

    /// # Errors
    ///
    /// Returns `ClusterStoreError` when the upload cannot be stored.
    pub async fn insert(&self, content: &[u8]) -> Result<Uuid, ClusterStoreError> {
        self.database
            .client()
            .await
            .map_err(ClusterStoreError::Unavailable)?
            .query_one(INSERT_UPLOAD, &[&content])
            .await
            .map_err(ClusterStoreError::InsertUpload)?
            .try_get("id")
            .map_err(ClusterStoreError::MalformedUploadRow)
    }
}
