use futures_util::TryFutureExt as _;
use uuid::Uuid;

use margaret::framework::active_record::json::Json;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::removal::Removal;
use margaret::framework::database::database::Database;
use margaret::framework::macros::model;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::authorization_grant::AuthorizationGrant;
use crate::authorization_grants_error::AuthorizationGrantsError;
use crate::pending_authorization::PendingAuthorization;
use crate::pending_authorization_take::PendingAuthorizationTake;

fn taken(
    PendingAuthorizationRecord {
        expires_at,
        grant,
        state,
        ..
    }: PendingAuthorizationRecord,
) -> PendingAuthorizationTake {
    PendingAuthorizationTake::Taken(Box::new(PendingAuthorization {
        expires_at: NumericDate::new(expires_at),
        grant: grant.into_payload(),
        state,
    }))
}

#[model(table = "pending_authorizations")]
pub struct PendingAuthorizationRecord {
    #[column(primary_key)]
    pub id: Uuid,
    #[column]
    #[index]
    pub expires_at: i64,
    #[column]
    pub grant: Json<AuthorizationGrant>,
    #[column]
    pub state: Option<String>,
}

impl PendingAuthorizationRecord {
    /// Holds the pending authorization until a single take removes it, and forgets every pending
    /// authorization that expired at or before `now`.
    ///
    /// # Errors
    ///
    /// Returns `AuthorizationGrantsError` when the expired pending authorizations cannot be swept
    /// or the pending authorization cannot be held.
    pub async fn hold(
        database: &Database,
        id: Uuid,
        PendingAuthorization {
            expires_at,
            grant,
            state,
        }: PendingAuthorization,
        now: NumericDate,
    ) -> Result<(), AuthorizationGrantsError> {
        let held = Self {
            expires_at: expires_at.seconds_since_epoch(),
            grant: Json::new(grant),
            id,
            state,
        };

        Self::query()
            .expires_at
            .at_most(now.seconds_since_epoch())
            .delete(database)
            .map_err(AuthorizationGrantsError::SweepPendingAuthorizations)
            .and_then(|_swept| {
                held.insert()
                    .run(database)
                    .map_err(AuthorizationGrantsError::HoldPendingAuthorization)
            })
            .await
    }

    /// Removes and returns the pending authorization exactly once across every instance.
    ///
    /// # Errors
    ///
    /// Returns `AuthorizationGrantsError::TakePendingAuthorization` when the pending
    /// authorization cannot be removed.
    pub async fn take(
        database: &Database,
        id: Uuid,
    ) -> Result<PendingAuthorizationTake, AuthorizationGrantsError> {
        Self::query()
            .id
            .eq(id)
            .delete(database)
            .await
            .map(|removal| match removal {
                Removal::Removed(record) => taken(record),
                Removal::Missing => PendingAuthorizationTake::Absent,
            })
            .map_err(AuthorizationGrantsError::TakePendingAuthorization)
    }
}
