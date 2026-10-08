use futures_util::TryFutureExt as _;

use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::authorization_grants_error::AuthorizationGrantsError;
use crate::refresh_family_record::RefreshFamilyRecord;
use crate::refresh_family_revocation_record::RefreshFamilyRevocationRecord;

pub(crate) async fn sweep_refresh_families(
    database: &Database,
    now: NumericDate,
) -> Result<(), AuthorizationGrantsError> {
    let now = now.seconds_since_epoch();

    RefreshFamilyRecord::query()
        .expires_at
        .at_most(now)
        .delete(database)
        .map_err(AuthorizationGrantsError::SweepRefreshFamilies)
        .and_then(|_swept| {
            RefreshFamilyRevocationRecord::query()
                .expires_at
                .at_most(now)
                .delete(database)
                .map_err(AuthorizationGrantsError::SweepRefreshFamilyRevocations)
        })
        .await
        .map(|_swept| ())
}
