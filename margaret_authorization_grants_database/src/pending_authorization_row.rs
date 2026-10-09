use tokio_postgres::Row;

use margaret_authorization_grants::pending_authorization::PendingAuthorization;
use margaret_authorization_grants_schema::pending_authorization_record::PendingAuthorizationRecord;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::authorization_grants_database_error::AuthorizationGrantsDatabaseError;
use crate::stored_grant::stored_grant;

fn pending_authorization_record(
    row: &Row,
) -> Result<PendingAuthorizationRecord, tokio_postgres::Error> {
    row.try_get("id").and_then(|id| {
        row.try_get("expires_at").and_then(|expires_at| {
            row.try_get("grant").and_then(|grant| {
                row.try_get("state")
                    .map(|state| PendingAuthorizationRecord {
                        expires_at,
                        grant,
                        id,
                        state,
                    })
            })
        })
    })
}

pub(crate) fn pending_authorization_row(
    row: &Row,
) -> Result<PendingAuthorization, AuthorizationGrantsDatabaseError> {
    pending_authorization_record(row)
        .map_err(AuthorizationGrantsDatabaseError::MalformedRow)
        .and_then(
            |PendingAuthorizationRecord {
                 expires_at,
                 grant,
                 state,
                 ..
             }| {
                stored_grant(&grant).map(|grant| PendingAuthorization {
                    expires_at: NumericDate::new(expires_at),
                    grant,
                    state,
                })
            },
        )
}
