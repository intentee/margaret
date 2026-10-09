use std::collections::BTreeSet;

use tokio_postgres::Row;

use margaret_authorization_grants::refresh_family::RefreshFamily;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants_schema::refresh_family_record::RefreshFamilyRecord;
use margaret_authorization_grants_schema::refresh_token_record::RefreshTokenRecord;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::authorization_grants_database_error::AuthorizationGrantsDatabaseError;

fn refresh_family_record(row: &Row) -> Result<RefreshFamilyRecord, tokio_postgres::Error> {
    row.try_get("family").and_then(|family| {
        row.try_get("auth_time").and_then(|auth_time| {
            row.try_get("client_id").and_then(|client_id| {
                row.try_get("expires_at").and_then(|expires_at| {
                    row.try_get("scopes").and_then(|scopes| {
                        row.try_get("subject").map(|subject| RefreshFamilyRecord {
                            auth_time,
                            client_id,
                            expires_at,
                            family,
                            scopes,
                            subject,
                        })
                    })
                })
            })
        })
    })
}

fn refresh_token_record(row: &Row) -> Result<RefreshTokenRecord, tokio_postgres::Error> {
    row.try_get("token").and_then(|token| {
        row.try_get("family").and_then(|family| {
            row.try_get("current").map(|current| RefreshTokenRecord {
                current,
                family,
                token,
            })
        })
    })
}

fn refresh_family(
    RefreshFamilyRecord {
        auth_time,
        client_id,
        expires_at,
        scopes,
        subject,
        ..
    }: RefreshFamilyRecord,
) -> Result<RefreshFamily, AuthorizationGrantsDatabaseError> {
    serde_json::from_str::<BTreeSet<Scope>>(&scopes)
        .map(|scopes| RefreshFamily {
            auth_time,
            client_id,
            expires_at: NumericDate::new(expires_at),
            scopes,
            subject,
        })
        .map_err(AuthorizationGrantsDatabaseError::MalformedScopes)
}

struct TokenOfFamily {
    family: RefreshFamilyRecord,
    token: RefreshTokenRecord,
}

pub(crate) fn refresh_token_lookup_row(
    row: &Row,
) -> Result<RefreshTokenLookup, AuthorizationGrantsDatabaseError> {
    refresh_token_record(row)
        .and_then(|token| refresh_family_record(row).map(|family| TokenOfFamily { family, token }))
        .map_err(AuthorizationGrantsDatabaseError::MalformedRow)
        .and_then(|TokenOfFamily { family, token }| {
            if token.current {
                refresh_family(family).map(|record| RefreshTokenLookup::Current {
                    family: token.family,
                    record,
                })
            } else {
                Ok(RefreshTokenLookup::Superseded {
                    family: token.family,
                })
            }
        })
}
