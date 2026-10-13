use std::time::Duration;

use chrono::DateTime;
use chrono::Utc;
use futures_util::TryFutureExt as _;
use uuid::Uuid;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::macros::model;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

use crate::session::Session;
use crate::session_lifetime_secs::SESSION_LIFETIME_SECS;
use crate::sessions_error::SessionsError;

#[model(table = "sessions")]
pub struct SessionRecord {
    #[column(primary_key, byte_length = 32)]
    pub secret: Vec<u8>,
    #[column]
    pub authenticated_at: DateTime<Utc>,
    #[column]
    #[index]
    pub expires_at: i64,
    #[column(unique)]
    pub id: Uuid,
    #[column]
    pub subject: Uuid,
}

impl SessionRecord {
    /// # Errors
    ///
    /// Returns `SessionsError::FindSession` when the session cannot be looked up.
    pub async fn current(
        database: &Database,
        secret: TokenDigest,
        now: NumericDate,
    ) -> Result<Lookup<Session>, SessionsError> {
        Self::query()
            .secret
            .eq(secret.as_bytes().to_vec())
            .find(database)
            .map_err(SessionsError::FindSession)
            .await
            .map(|lookup| match lookup {
                Lookup::Found(Self {
                    authenticated_at,
                    expires_at,
                    id,
                    subject,
                    ..
                }) if expires_at > now.seconds_since_epoch() => Lookup::Found(Session {
                    authenticated_at,
                    id,
                    subject,
                }),
                Lookup::Found(_) | Lookup::Missing => Lookup::Missing,
            })
    }

    /// # Errors
    ///
    /// Returns `SessionsError::ForgetSession` when the session cannot be forgotten.
    pub async fn forget(database: &Database, secret: TokenDigest) -> Result<(), SessionsError> {
        Self::query()
            .secret
            .eq(secret.as_bytes().to_vec())
            .delete(database)
            .map_err(SessionsError::ForgetSession)
            .await
            .map(|_removal| ())
    }

    /// # Errors
    ///
    /// Returns `SessionsError::SweepSessions` when the expired sessions cannot be swept, and
    /// `SessionsError::OpenSession` when the session cannot be stored.
    pub async fn open(
        database: &Database,
        secret: TokenDigest,
        Session {
            authenticated_at,
            id,
            subject,
        }: Session,
        now: NumericDate,
    ) -> Result<(), SessionsError> {
        Self::query()
            .expires_at
            .at_most(now.seconds_since_epoch())
            .delete(database)
            .map_err(SessionsError::SweepSessions)
            .await?;

        Self {
            authenticated_at,
            expires_at: now
                .after(Duration::from_secs(u64::from(SESSION_LIFETIME_SECS)))
                .seconds_since_epoch(),
            id,
            secret: secret.as_bytes().to_vec(),
            subject,
        }
        .insert()
        .run(database)
        .map_err(SessionsError::OpenSession)
        .await
    }
}
