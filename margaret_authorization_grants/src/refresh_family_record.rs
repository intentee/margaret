use std::collections::BTreeSet;
use std::future::ready;

use chrono::DateTime;
use chrono::Utc;
use futures_util::TryFutureExt as _;
use uuid::Uuid;

use margaret::framework::active_record::detached::Detached;
use margaret::framework::active_record::guarded_insertion::GuardedInsertion;
use margaret::framework::active_record::json::Json;
use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::database::isolation::Isolation;
use margaret::framework::database::transaction::Transaction;
use margaret::framework::macros::model;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

use crate::authorization_grants_error::AuthorizationGrantsError;
use crate::family_opening::FamilyOpening;
use crate::refresh_family::RefreshFamily;
use crate::refresh_family_revocation_record::RefreshFamilyRevocationRecord;
use crate::refresh_token_record::RefreshTokenRecord;
use crate::sweep_refresh_families::sweep_refresh_families;

async fn opened_within(
    transaction: Transaction<'_>,
    opened: &RefreshFamilyRecord,
    first_token: &RefreshTokenRecord,
    now: NumericDate,
) -> Result<FamilyOpening, AuthorizationGrantsError> {
    let insertion = opened
        .insert()
        .when(
            RefreshFamilyRevocationRecord::query()
                .family
                .eq(opened.id)
                .when(|revocation| revocation.expires_at.above(now.seconds_since_epoch()))
                .exists::<Detached>()
                .negated(),
        )
        .run(&transaction)
        .await
        .map_err(AuthorizationGrantsError::OpenRefreshFamily);

    ready(insertion)
        .and_then(|insertion| async move {
            match insertion {
                GuardedInsertion::Inserted => {
                    let issued = first_token
                        .insert()
                        .run(&transaction)
                        .await
                        .map_err(AuthorizationGrantsError::IssueFirstRefreshToken);

                    ready(issued)
                        .and_then(|()| {
                            transaction
                                .commit()
                                .map_err(AuthorizationGrantsError::CommitRefreshFamilyOpening)
                        })
                        .await
                        .map(|()| FamilyOpening::Opened)
                }
                GuardedInsertion::Refused => transaction
                    .rollback()
                    .await
                    .map(|()| FamilyOpening::Revoked)
                    .map_err(AuthorizationGrantsError::RollbackRevokedRefreshFamilyOpening),
            }
        })
        .await
}

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
    /// Opens the family with its first token, atomically across every instance, unless the family
    /// was revoked before, which it reports as revoked; forgets every family, with its tokens, and
    /// every revocation that expired at or before `now`.
    ///
    /// # Errors
    ///
    /// Returns `AuthorizationGrantsError` when the expired families cannot be swept, or the
    /// family cannot be opened with its first token.
    pub async fn open(
        database: &Database,
        family: Uuid,
        RefreshFamily {
            auth_time,
            client_id,
            expires_at,
            scopes,
            subject,
        }: RefreshFamily,
        first_token: TokenDigest,
        now: NumericDate,
    ) -> Result<FamilyOpening, AuthorizationGrantsError> {
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
            family: Key::new(family),
            token: first_token.as_bytes().to_vec(),
        };

        sweep_refresh_families(database, now)
            .and_then(|()| {
                database
                    .connection()
                    .map_err(AuthorizationGrantsError::BeginRefreshFamilyOpening)
            })
            .and_then(|mut connection| async move {
                let began = connection
                    .transaction(Isolation::ReadCommitted)
                    .await
                    .map_err(AuthorizationGrantsError::BeginRefreshFamilyOpening);

                ready(began)
                    .and_then(|transaction| opened_within(transaction, &opened, &first_token, now))
                    .await
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
