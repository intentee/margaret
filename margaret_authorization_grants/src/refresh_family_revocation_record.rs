use uuid::Uuid;

use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::macros::model;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::authorization_grants_error::AuthorizationGrantsError;
use crate::refresh_family_lifetime::REFRESH_FAMILY_LIFETIME;

#[model(table = "refresh_family_revocations")]
pub struct RefreshFamilyRevocationRecord {
    #[column(primary_key)]
    pub family: Uuid,
    #[column]
    #[index]
    pub expires_at: i64,
}

impl RefreshFamilyRevocationRecord {
    /// Revokes the family until it expires; a family that is not opened yet stays revoked for the
    /// refresh family lifetime after `now`.
    ///
    /// # Errors
    ///
    /// Returns `AuthorizationGrantsError::RevokeRefreshFamily` when the revocation cannot be
    /// written.
    pub async fn revoke(
        database: &Database,
        family: Uuid,
        now: NumericDate,
    ) -> Result<(), AuthorizationGrantsError> {
        Self {
            expires_at: now.after(REFRESH_FAMILY_LIFETIME).seconds_since_epoch(),
            family,
        }
        .insert()
        .or_update(|columns| columns.expires_at.greatest())
        .run(database)
        .await
        .map(|_written| ())
        .map_err(AuthorizationGrantsError::RevokeRefreshFamily)
    }
}
