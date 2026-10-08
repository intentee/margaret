use std::future::ready;

use futures_util::TryFutureExt as _;
use uuid::Uuid;

use margaret::framework::active_record::change::Change;
use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::database::executor::Executor;
use margaret::framework::database::isolation::Isolation;
use margaret::framework::database::transaction::Transaction;
use margaret::framework::macros::model;
use margaret::framework::model::on_delete::OnDelete;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

use crate::authorization_grants_error::AuthorizationGrantsError;
use crate::refresh_family_record::RefreshFamilyRecord;
use crate::refresh_family_revocation_record::RefreshFamilyRevocationRecord;
use crate::refresh_rotation::RefreshRotation;
use crate::refresh_token_lookup::RefreshTokenLookup;
use crate::refresh_token_with_family::RefreshTokenWithFamily;
use crate::sweep_refresh_families::sweep_refresh_families;

async fn resolved(
    database: &Database,
    RefreshTokenWithFamily { family, token }: RefreshTokenWithFamily,
) -> Result<RefreshTokenLookup, AuthorizationGrantsError> {
    RefreshFamilyRevocationRecord::query()
        .family
        .eq(family.id)
        .find(database)
        .await
        .map(|revocation| match revocation {
            Lookup::Found(_) => RefreshTokenLookup::Unknown,
            Lookup::Missing if token.current => RefreshTokenLookup::Current {
                family: family.id,
                record: family.into_family(),
            },
            Lookup::Missing => RefreshTokenLookup::Superseded { family: family.id },
        })
        .map_err(AuthorizationGrantsError::FindRefreshFamilyRevocation)
}

async fn rotated_in_family(
    transaction: Transaction<'_>,
    family: Uuid,
    next: Vec<u8>,
    now: NumericDate,
) -> Result<RefreshRotation, AuthorizationGrantsError> {
    let open_family = RefreshFamilyRecord::query()
        .id
        .eq(family)
        .when(|stored| {
            stored.expires_at.above(now.seconds_since_epoch()).and(
                RefreshFamilyRevocationRecord::query()
                    .family
                    .eq(family)
                    .exists::<RefreshFamilyRecord>()
                    .negated(),
            )
        })
        .find(&transaction)
        .await
        .map_err(AuthorizationGrantsError::FindRotatedRefreshFamily);

    ready(open_family)
        .and_then(|lookup| async move {
            match lookup {
                Lookup::Found(_) => {
                    let next_token = RefreshTokenRecord {
                        current: true,
                        family: Key::new(family),
                        token: next,
                    };
                    let issued = next_token
                        .insert()
                        .run(&transaction)
                        .await
                        .map_err(AuthorizationGrantsError::IssueNextRefreshToken);

                    ready(issued)
                        .and_then(|()| {
                            transaction
                                .commit()
                                .map_err(AuthorizationGrantsError::CommitRefreshTokenRotation)
                        })
                        .await
                        .map(|()| RefreshRotation::Rotated)
                }
                Lookup::Missing => transaction
                    .rollback()
                    .await
                    .map(|()| RefreshRotation::Unknown)
                    .map_err(AuthorizationGrantsError::RollbackClosedRefreshTokenRotation),
            }
        })
        .await
}

async fn rotated_within(
    transaction: Transaction<'_>,
    presented: Vec<u8>,
    next: Vec<u8>,
    now: NumericDate,
) -> Result<RefreshRotation, AuthorizationGrantsError> {
    let locked = RefreshTokenRecord::query()
        .token
        .eq(presented.clone())
        .when(|stored| stored.current.eq(true))
        .update(&transaction, |columns| columns.current.to(false))
        .await
        .map_err(AuthorizationGrantsError::RotateRefreshToken);

    ready(locked)
        .and_then(|change| async move {
            match change {
                Change::Changed(RefreshTokenRecord { family, .. }) => {
                    rotated_in_family(transaction, family.into_primary_key(), next, now).await
                }
                Change::Unmatched => {
                    let rotation = superseded(&transaction, presented).await;

                    ready(rotation)
                        .and_then(|rotation| {
                            transaction.rollback().map_ok(move |()| rotation).map_err(
                                AuthorizationGrantsError::RollbackUnmatchedRefreshTokenRotation,
                            )
                        })
                        .await
                }
            }
        })
        .await
}

async fn superseded<Executing: Executor>(
    executor: &Executing,
    presented: Vec<u8>,
) -> Result<RefreshRotation, AuthorizationGrantsError> {
    RefreshTokenRecord::query()
        .token
        .eq(presented)
        .when(|stored| stored.current.eq(false))
        .find(executor)
        .map_err(AuthorizationGrantsError::FindSupersededRefreshToken)
        .and_then(|lookup| async move {
            match lookup {
                Lookup::Found(RefreshTokenRecord { family, .. }) => {
                    let family = family.into_primary_key();

                    RefreshFamilyRevocationRecord::query()
                        .family
                        .eq(family)
                        .find(executor)
                        .await
                        .map(|revocation| match revocation {
                            Lookup::Found(_) => RefreshRotation::Unknown,
                            Lookup::Missing => RefreshRotation::Superseded { family },
                        })
                        .map_err(AuthorizationGrantsError::FindSupersededRefreshFamilyRevocation)
                }
                Lookup::Missing => Ok(RefreshRotation::Unknown),
            }
        })
        .await
}

#[model(table = "refresh_tokens")]
pub struct RefreshTokenRecord {
    #[column(primary_key, byte_length = 32)]
    pub token: Vec<u8>,
    #[column]
    #[foreign_key(on_delete = OnDelete::Cascade)]
    #[index]
    pub family: Key<RefreshFamilyRecord>,
    #[column]
    pub current: bool,
}

impl RefreshTokenRecord {
    /// Resolves the token to its family while the family is open, reporting a token that a
    /// rotation replaced as superseded.
    ///
    /// # Errors
    ///
    /// Returns `AuthorizationGrantsError` when the token or the revocation of its family cannot
    /// be found.
    pub async fn lookup(
        database: &Database,
        token: TokenDigest,
    ) -> Result<RefreshTokenLookup, AuthorizationGrantsError> {
        Self::query()
            .token
            .eq(token.as_bytes().to_vec())
            .load::<RefreshTokenWithFamily, _>(database)
            .map_err(AuthorizationGrantsError::FindRefreshToken)
            .and_then(|lookup| async move {
                match lookup {
                    Lookup::Found(found) => resolved(database, found).await,
                    Lookup::Missing => Ok(RefreshTokenLookup::Unknown),
                }
            })
            .await
    }

    /// Replaces the presented current token with the next one exactly once across every instance,
    /// reporting every other rotation of the same token as superseded; forgets every family, with
    /// its tokens, and every revocation that expired at or before `now`.
    ///
    /// # Errors
    ///
    /// Returns `AuthorizationGrantsError` when the expired families cannot be swept, or the token
    /// cannot be rotated.
    pub async fn rotate(
        database: &Database,
        presented: TokenDigest,
        next: TokenDigest,
        now: NumericDate,
    ) -> Result<RefreshRotation, AuthorizationGrantsError> {
        let presented = presented.as_bytes().to_vec();
        let next = next.as_bytes().to_vec();

        sweep_refresh_families(database, now)
            .and_then(|()| {
                database
                    .connection()
                    .map_err(AuthorizationGrantsError::BeginRefreshTokenRotation)
            })
            .and_then(|mut connection| async move {
                let began = connection
                    .transaction(Isolation::ReadCommitted)
                    .await
                    .map_err(AuthorizationGrantsError::BeginRefreshTokenRotation);

                ready(began)
                    .and_then(|transaction| rotated_within(transaction, presented, next, now))
                    .await
            })
            .await
    }
}
