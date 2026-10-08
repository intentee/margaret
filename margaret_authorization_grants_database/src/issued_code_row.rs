use tokio_postgres::Row;

use margaret_authorization_grants::issued_code::IssuedCode;
use margaret_authorization_grants_schema::authorization_code_record::AuthorizationCodeRecord;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::authorization_grants_database_error::AuthorizationGrantsDatabaseError;
use crate::stored_grant::stored_grant;

fn authorization_code_record(row: &Row) -> Result<AuthorizationCodeRecord, tokio_postgres::Error> {
    row.try_get("code").and_then(|code| {
        row.try_get("expires_at").and_then(|expires_at| {
            row.try_get("grant").and_then(|grant| {
                row.try_get("redeemed_by")
                    .map(|redeemed_by| AuthorizationCodeRecord {
                        code,
                        expires_at,
                        grant,
                        redeemed_by,
                    })
            })
        })
    })
}

pub(crate) fn issued_code_row(row: &Row) -> Result<IssuedCode, AuthorizationGrantsDatabaseError> {
    authorization_code_record(row)
        .map_err(AuthorizationGrantsDatabaseError::MalformedRow)
        .and_then(
            |AuthorizationCodeRecord {
                 expires_at, grant, ..
             }| {
                stored_grant(&grant).map(|grant| IssuedCode {
                    expires_at: NumericDate::new(expires_at),
                    grant,
                })
            },
        )
}
