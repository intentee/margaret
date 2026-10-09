use std::collections::BTreeSet;

use chrono::DateTime;
use chrono::Utc;
use futures_util::TryFutureExt as _;
use uuid::Uuid;

use margaret::framework::active_record::json::Json;
use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::database::transaction::Transaction;
use margaret::framework::macros::model;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::authorization_grants_error::AuthorizationGrantsError;
use crate::family_opening::FamilyOpening;
use crate::refresh_family::RefreshFamily;
use crate::refresh_token_record::RefreshTokenRecord;

#[model(table = "refresh_families")]
pub struct RefreshFamilyRecord {
    #[column(primary_key)]
    pub id: Uuid,
    #[column]
    pub auth_time: DateTime<Utc>,
    #[column]
    pub client_id: String,
    #[column]
    #[index]
    pub expires_at: i64,
    #[column]
    pub scopes: Json<BTreeSet<Scope>>,
    #[column]
    pub subject: Uuid,
}

impl RefreshFamilyRecord {
    /// Revokes the family by forgetting it with every refresh token of it.
    ///
    /// # Errors
    ///
    /// Returns `AuthorizationGrantsError::RevokeRefreshFamily` when the family cannot be
    /// forgotten.
    pub async fn revoke(database: &Database, family: Uuid) -> Result<(), AuthorizationGrantsError> {
        Self::query()
            .id
            .eq(family)
            .delete(database)
            .map_err(AuthorizationGrantsError::RevokeRefreshFamily)
            .await
            .map(|_removal| ())
    }

    pub(crate) async fn open(
        transaction: &Transaction<'_>,
        family: Uuid,
        FamilyOpening {
            family:
                RefreshFamily {
                    auth_time,
                    client_id,
                    expires_at,
                    scopes,
                    subject,
                },
            first_token,
        }: FamilyOpening,
    ) -> Result<(), AuthorizationGrantsError> {
        let opened = Self {
            auth_time,
            client_id,
            expires_at: expires_at.seconds_since_epoch(),
            id: family,
            scopes: Json::new(scopes),
            subject,
        };
        let first_token = RefreshTokenRecord {
            current: true,
            family: Key::of(&opened),
            token: first_token.as_bytes().to_vec(),
        };

        opened
            .insert()
            .run(transaction)
            .map_err(AuthorizationGrantsError::OpenRefreshFamily)
            .and_then(|()| {
                first_token
                    .insert()
                    .run(transaction)
                    .map_err(AuthorizationGrantsError::IssueFirstRefreshToken)
            })
            .await
    }

    pub(crate) fn into_family(self) -> RefreshFamily {
        RefreshFamily {
            auth_time: self.auth_time,
            client_id: self.client_id,
            expires_at: NumericDate::new(self.expires_at),
            scopes: self.scopes.into_payload(),
            subject: self.subject,
        }
    }
}
