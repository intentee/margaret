use std::sync::Arc;

use uuid::Uuid;

use margaret::framework::database::database::Database;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::tokio_postgres::Row;

use crate::models::user::User;
use crate::stores::cluster_store_error::ClusterStoreError;
use crate::system_clock::SystemClock;

const FIND_SESSION_USER: &str = "SELECT user_sessions.authenticated_at, users.id, users.name \
     FROM user_sessions JOIN users ON users.id = user_sessions.user_id \
     WHERE user_sessions.id = $1";

const FIND_USER_NAME: &str = "SELECT name FROM users WHERE id = $1";

const START_SESSION: &str = "INSERT INTO user_sessions (id, authenticated_at, user_id) \
     SELECT $1, $2, users.id FROM users WHERE users.id = $3";

fn session_user(row: &Row) -> Result<User, ClusterStoreError> {
    Ok(User {
        authenticated_at: row
            .try_get("authenticated_at")
            .map_err(ClusterStoreError::MalformedSessionRow)?,
        id: row
            .try_get("id")
            .map_err(ClusterStoreError::MalformedSessionRow)?,
        name: row
            .try_get("name")
            .map_err(ClusterStoreError::MalformedSessionRow)?,
    })
}

#[singleton]
pub struct UserStore {
    clock: Arc<SystemClock>,
    database: Arc<Database>,
}

impl UserStore {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(clock: Arc<SystemClock>, database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { clock, database })
    }

    /// # Errors
    ///
    /// Returns `ClusterStoreError` when the session cannot be read.
    pub async fn find_user_by_session(
        &self,
        session: Uuid,
    ) -> Result<Option<User>, ClusterStoreError> {
        self.database
            .client()
            .await
            .map_err(ClusterStoreError::Unavailable)?
            .query_opt(FIND_SESSION_USER, &[&session])
            .await
            .map_err(ClusterStoreError::FindSessionUser)?
            .as_ref()
            .map(session_user)
            .transpose()
    }

    /// # Errors
    ///
    /// Returns `ClusterStoreError` when the user cannot be read.
    pub async fn find_user_name(&self, id: Uuid) -> Result<Option<String>, ClusterStoreError> {
        self.database
            .client()
            .await
            .map_err(ClusterStoreError::Unavailable)?
            .query_opt(FIND_USER_NAME, &[&id])
            .await
            .map_err(ClusterStoreError::FindUserName)?
            .map(|row| {
                row.try_get("name")
                    .map_err(ClusterStoreError::MalformedUserRow)
            })
            .transpose()
    }

    /// # Errors
    ///
    /// Returns `ClusterStoreError` when the session cannot be stored.
    pub async fn start_session(&self, user_id: Uuid) -> Result<Option<Uuid>, ClusterStoreError> {
        let session = Uuid::new_v4();

        self.database
            .client()
            .await
            .map_err(ClusterStoreError::Unavailable)?
            .execute(START_SESSION, &[&session, &self.clock.now(), &user_id])
            .await
            .map_err(ClusterStoreError::StartSession)
            .map(|started| (started == 1).then_some(session))
    }
}
