use futures_util::TryFutureExt as _;

use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::authorization_grants_error::AuthorizationGrantsError;
use crate::refresh_family_record::RefreshFamilyRecord;

pub(crate) async fn sweep_refresh_families(
    database: &Database,
    now: NumericDate,
) -> Result<(), AuthorizationGrantsError> {
    RefreshFamilyRecord::query()
        .expires_at
        .at_most(now.seconds_since_epoch())
        .delete(database)
        .map_err(AuthorizationGrantsError::SweepRefreshFamilies)
        .await
        .map(|_swept| ())
}
