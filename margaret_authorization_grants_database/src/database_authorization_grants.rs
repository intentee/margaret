use std::future::ready;
use std::sync::Arc;

use async_trait::async_trait;
use futures_util::TryFutureExt as _;
use uuid::Uuid;

use margaret_authorization_grants::code_redemption::CodeRedemption;
use margaret_authorization_grants::family_opening::FamilyOpening;
use margaret_authorization_grants::issued_code::IssuedCode;
use margaret_authorization_grants::pending_authorization::PendingAuthorization;
use margaret_authorization_grants::pending_authorization_take::PendingAuthorizationTake;
use margaret_authorization_grants::refresh_family::RefreshFamily;
use margaret_authorization_grants::refresh_family_lifetime::REFRESH_FAMILY_LIFETIME;
use margaret_authorization_grants::refresh_rotation::RefreshRotation;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;
use margaret_database::database::Database;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

use crate::authorization_grants_database_error::AuthorizationGrantsDatabaseError;
use crate::authorization_grants_statements::AuthorizationGrantsStatements;
use crate::issued_code_row::issued_code_row;
use crate::pending_authorization_row::pending_authorization_row;
use crate::refresh_token_lookup_row::refresh_token_lookup_row;

pub struct DatabaseAuthorizationGrants {
    database: Arc<Database>,
    statements: AuthorizationGrantsStatements,
}

impl DatabaseAuthorizationGrants {
    #[must_use]
    pub fn create(database: Arc<Database>) -> Self {
        Self {
            database,
            statements: AuthorizationGrantsStatements::new(),
        }
    }
}

#[async_trait]
impl StoresAuthorizationGrants for DatabaseAuthorizationGrants {
    async fn find_refresh_token(&self, token: TokenDigest) -> anyhow::Result<RefreshTokenLookup> {
        Ok(self
            .database
            .client()
            .map_err(AuthorizationGrantsDatabaseError::Unavailable)
            .and_then(|client| async move {
                client
                    .query_opt(
                        &self.statements.find_refresh_token,
                        &[&token.as_bytes().as_slice()],
                    )
                    .await
                    .map_err(AuthorizationGrantsDatabaseError::FindRefreshToken)
            })
            .await
            .and_then(|row| match row {
                None => Ok(RefreshTokenLookup::Unknown),
                Some(row) => refresh_token_lookup_row(&row),
            })?)
    }

    async fn hold_pending_authorization(
        &self,
        id: Uuid,
        PendingAuthorization {
            expires_at,
            grant,
            state,
        }: PendingAuthorization,
        now: NumericDate,
    ) -> anyhow::Result<()> {
        ready(
            serde_json::to_string(&grant)
                .map_err(AuthorizationGrantsDatabaseError::GrantSerialization),
        )
        .and_then(|grant| async move {
            self.database
                .client()
                .map_err(AuthorizationGrantsDatabaseError::Unavailable)
                .and_then(|client| async move {
                    client
                        .execute(
                            &self.statements.hold_pending_authorization,
                            &[
                                &id,
                                &expires_at.seconds_since_epoch(),
                                &grant,
                                &state,
                                &now.seconds_since_epoch(),
                            ],
                        )
                        .await
                        .map_err(AuthorizationGrantsDatabaseError::HoldPendingAuthorization)
                })
                .await
        })
        .await?;

        Ok(())
    }

    async fn issue_code(
        &self,
        code: TokenDigest,
        IssuedCode { expires_at, grant }: IssuedCode,
        now: NumericDate,
    ) -> anyhow::Result<()> {
        ready(
            serde_json::to_string(&grant)
                .map_err(AuthorizationGrantsDatabaseError::GrantSerialization),
        )
        .and_then(|grant| async move {
            self.database
                .client()
                .map_err(AuthorizationGrantsDatabaseError::Unavailable)
                .and_then(|client| async move {
                    client
                        .execute(
                            &self.statements.issue_code,
                            &[
                                &code.as_bytes().as_slice(),
                                &expires_at.seconds_since_epoch(),
                                &grant,
                                &now.seconds_since_epoch(),
                            ],
                        )
                        .await
                        .map_err(AuthorizationGrantsDatabaseError::IssueCode)
                })
                .await
        })
        .await?;

        Ok(())
    }

    async fn open_refresh_family(
        &self,
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
    ) -> anyhow::Result<FamilyOpening> {
        Ok(ready(
            serde_json::to_string(&scopes)
                .map_err(AuthorizationGrantsDatabaseError::ScopesSerialization),
        )
        .and_then(|scopes| async move {
            self.database
                .client()
                .map_err(AuthorizationGrantsDatabaseError::Unavailable)
                .and_then(|client| async move {
                    client
                        .execute(
                            &self.statements.open_refresh_family,
                            &[
                                &family,
                                &auth_time,
                                &client_id,
                                &expires_at.seconds_since_epoch(),
                                &scopes,
                                &subject,
                                &first_token.as_bytes().as_slice(),
                                &now.seconds_since_epoch(),
                            ],
                        )
                        .await
                        .map_err(AuthorizationGrantsDatabaseError::OpenRefreshFamily)
                })
                .await
        })
        .await
        .map(|opened| match opened {
            0 => FamilyOpening::Revoked,
            _ => FamilyOpening::Opened,
        })?)
    }

    async fn redeem_code(&self, code: TokenDigest, family: Uuid) -> anyhow::Result<CodeRedemption> {
        let code = code.as_bytes().as_slice();

        Ok(self
            .database
            .client()
            .map_err(AuthorizationGrantsDatabaseError::Unavailable)
            .and_then(|client| async move {
                match client
                    .query_opt(&self.statements.redeem_code, &[&code, &family])
                    .await
                    .map_err(AuthorizationGrantsDatabaseError::RedeemCode)
                {
                    Ok(Some(redeemed)) => issued_code_row(&redeemed)
                        .map(|issued| CodeRedemption::Redeemed(Box::new(issued))),
                    Ok(None) => client
                        .query_opt(&self.statements.find_redeemed_code, &[&code])
                        .await
                        .map_err(AuthorizationGrantsDatabaseError::RedeemCode)
                        .and_then(|row| match row {
                            None => Ok(CodeRedemption::Unknown),
                            Some(row) => row
                                .try_get("redeemed_by")
                                .map(|family| CodeRedemption::AlreadyRedeemed { family })
                                .map_err(AuthorizationGrantsDatabaseError::MalformedRow),
                        }),
                    Err(error) => Err(error),
                }
            })
            .await?)
    }

    async fn revoke_refresh_family(&self, family: Uuid, now: NumericDate) -> anyhow::Result<()> {
        self.database
            .client()
            .map_err(AuthorizationGrantsDatabaseError::Unavailable)
            .and_then(|client| async move {
                client
                    .execute(
                        &self.statements.revoke_refresh_family,
                        &[
                            &family,
                            &now.after(REFRESH_FAMILY_LIFETIME).seconds_since_epoch(),
                            &now.seconds_since_epoch(),
                        ],
                    )
                    .await
                    .map_err(AuthorizationGrantsDatabaseError::RevokeRefreshFamily)
            })
            .await?;

        Ok(())
    }

    async fn rotate_refresh_token(
        &self,
        presented: TokenDigest,
        next: TokenDigest,
        now: NumericDate,
    ) -> anyhow::Result<RefreshRotation> {
        let presented = presented.as_bytes().as_slice();

        Ok(self
            .database
            .client()
            .map_err(AuthorizationGrantsDatabaseError::Unavailable)
            .and_then(|client| async move {
                match client
                    .execute(
                        &self.statements.rotate_refresh_token,
                        &[
                            &presented,
                            &next.as_bytes().as_slice(),
                            &now.seconds_since_epoch(),
                        ],
                    )
                    .await
                    .map_err(AuthorizationGrantsDatabaseError::RotateRefreshToken)
                {
                    Ok(0) => client
                        .query_opt(
                            &self.statements.find_superseded_refresh_token,
                            &[&presented],
                        )
                        .await
                        .map_err(AuthorizationGrantsDatabaseError::RotateRefreshToken)
                        .and_then(|row| match row {
                            None => Ok(RefreshRotation::Unknown),
                            Some(row) => row
                                .try_get("family")
                                .map(|family| RefreshRotation::Superseded { family })
                                .map_err(AuthorizationGrantsDatabaseError::MalformedRow),
                        }),
                    Ok(_) => Ok(RefreshRotation::Rotated),
                    Err(error) => Err(error),
                }
            })
            .await?)
    }

    async fn take_pending_authorization(
        &self,
        id: Uuid,
    ) -> anyhow::Result<PendingAuthorizationTake> {
        Ok(self
            .database
            .client()
            .map_err(AuthorizationGrantsDatabaseError::Unavailable)
            .and_then(|client| async move {
                client
                    .query_opt(&self.statements.take_pending_authorization, &[&id])
                    .await
                    .map_err(AuthorizationGrantsDatabaseError::TakePendingAuthorization)
            })
            .await
            .and_then(|row| match row {
                None => Ok(PendingAuthorizationTake::Absent),
                Some(row) => pending_authorization_row(&row)
                    .map(|pending| PendingAuthorizationTake::Taken(Box::new(pending))),
            })?)
    }
}
