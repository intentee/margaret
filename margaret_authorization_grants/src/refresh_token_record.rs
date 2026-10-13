use std::future::ready;

use futures_util::TryFutureExt as _;
use uuid::Uuid;

use margaret::framework::active_record::change::Change;
use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::database::isolation::Isolation;
use margaret::framework::database::transaction::Transaction;
use margaret::framework::macros::model;
use margaret::framework::model::on_delete::OnDelete;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

use crate::authorization_grants_error::AuthorizationGrantsError;
use crate::refresh_family_record::RefreshFamilyRecord;
use crate::refresh_rotation::RefreshRotation;
use crate::refresh_token_lookup::RefreshTokenLookup;
use crate::refresh_token_with_family::RefreshTokenWithFamily;
use crate::sweep_refresh_families::sweep_refresh_families;

fn resolved(
    RefreshTokenWithFamily { family, token }: RefreshTokenWithFamily,
) -> RefreshTokenLookup {
    if token.current {
        RefreshTokenLookup::Current {
            family: family.id,
            record: family.into_family(),
        }
    } else {
        RefreshTokenLookup::Superseded {
            client_id: family.client_id,
            family: family.id,
        }
    }
}

async fn rotated_token(
    transaction: Transaction<'_>,
    family: Uuid,
    presented: Vec<u8>,
    next: Vec<u8>,
) -> Result<RefreshRotation, AuthorizationGrantsError> {
    let locked = RefreshTokenRecord::query()
        .token
        .eq(presented)
        .when(|stored| stored.current.eq(true))
        .update(&transaction, |columns| columns.current.to(false))
        .await
        .map_err(AuthorizationGrantsError::RotateRefreshToken);

    ready(locked)
        .and_then(|change| async move {
            match change {
                Change::Changed(_) => {
                    let issued = RefreshTokenRecord {
                        current: true,
                        family: Key::new(family),
                        token: next,
                    }
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
                Change::Unmatched => transaction
                    .rollback()
                    .await
                    .map(|()| RefreshRotation::Superseded { family })
                    .map_err(AuthorizationGrantsError::RollbackUnmatchedRefreshTokenRotation),
            }
        })
        .await
}

async fn rotated_in_family(
    transaction: Transaction<'_>,
    family: Uuid,
    presented: Vec<u8>,
    next: Vec<u8>,
    now: NumericDate,
) -> Result<RefreshRotation, AuthorizationGrantsError> {
    let held = RefreshFamilyRecord::query()
        .id
        .eq(family)
        .when(|stored| stored.expires_at.above(now.seconds_since_epoch()))
        .find_key_shared(&transaction)
        .await
        .map_err(AuthorizationGrantsError::HoldRotatedRefreshFamily);

    ready(held)
        .and_then(|lookup| async move {
            match lookup {
                Lookup::Found(_) => rotated_token(transaction, family, presented, next).await,
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
    let found = RefreshTokenRecord::query()
        .token
        .eq(presented.clone())
        .find(&transaction)
        .await
        .map_err(AuthorizationGrantsError::FindPresentedRefreshToken);

    ready(found)
        .and_then(|lookup| async move {
            match lookup {
                Lookup::Found(RefreshTokenRecord { family, .. }) => {
                    rotated_in_family(transaction, family.into_primary_key(), presented, next, now)
                        .await
                }
                Lookup::Missing => transaction
                    .rollback()
                    .await
                    .map(|()| RefreshRotation::Unknown)
                    .map_err(AuthorizationGrantsError::RollbackUnknownRefreshTokenRotation),
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
    /// Resolves the token to its family, reporting a token that a rotation replaced as
    /// superseded.
    ///
    /// # Errors
    ///
    /// Returns `AuthorizationGrantsError::FindRefreshToken` when the token cannot be found with
    /// its family.
    pub async fn lookup(
        database: &Database,
        token: TokenDigest,
    ) -> Result<RefreshTokenLookup, AuthorizationGrantsError> {
        Self::query()
            .token
            .eq(token.as_bytes().to_vec())
            .load::<RefreshTokenWithFamily, _>(database)
            .map_err(AuthorizationGrantsError::FindRefreshToken)
            .await
            .map(|lookup| match lookup {
                Lookup::Found(found) => resolved(found),
                Lookup::Missing => RefreshTokenLookup::Unknown,
            })
    }

    /// Replaces the presented current token with the next one exactly once across every instance,
    /// reporting every other rotation of the same token as superseded. Holds the family of the
    /// token before the token itself, the order in which revoking and sweeping a family remove
    /// it with its tokens, so a rotation never waits on them while they wait on the rotation.
    /// Forgets every family, with its tokens, that expired at or before `now` first.
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
