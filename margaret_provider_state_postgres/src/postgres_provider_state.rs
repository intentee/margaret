use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::query;
use sqlx::query_as;
use sqlx::query_scalar;
use sqlx::types::Json;
use uuid::Uuid;

use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_provider_state_storage::authorization_code_lifetime::AUTHORIZATION_CODE_LIFETIME;
use margaret_provider_state_storage::authorization_grant::AuthorizationGrant;
use margaret_provider_state_storage::code_redemption::CodeRedemption;
use margaret_provider_state_storage::code_redemption_request::CodeRedemptionRequest;
use margaret_provider_state_storage::decided_authorization::DecidedAuthorization;
use margaret_provider_state_storage::pending_authorization::PendingAuthorization;
use margaret_provider_state_storage::pending_authorization_lifetime::PENDING_AUTHORIZATION_LIFETIME;
use margaret_provider_state_storage::pending_decision::PendingDecision;
use margaret_provider_state_storage::pending_verdict::PendingVerdict;
use margaret_provider_state_storage::provider_state_error::ProviderStateError;
use margaret_provider_state_storage::refresh_admission::RefreshAdmission;
use margaret_provider_state_storage::refresh_family::RefreshFamily;
use margaret_provider_state_storage::refresh_family_lifetime::REFRESH_FAMILY_LIFETIME;
use margaret_provider_state_storage::refresh_issuance::RefreshIssuance;
use margaret_provider_state_storage::refresh_revocation::RefreshRevocation;
use margaret_provider_state_storage::refresh_rotation::RefreshRotation;
use margaret_provider_state_storage::refresh_scope::RefreshScope;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;
use margaret_provider_state_storage::token_digest::TokenDigest;

use crate::decided_row::DecidedRow;
use crate::provider_state_statements::ProviderStateStatements;
use crate::redeemed_row::RedeemedRow;
use crate::rotated_row::RotatedRow;

fn refresh_family_of(id: Uuid, grant: AuthorizationGrant) -> RefreshFamily {
    RefreshFamily {
        auth_time: grant.auth_time,
        client_id: grant.client_id,
        id,
        scopes: grant.scopes,
        subject: grant.subject,
    }
}

pub struct PostgresProviderState {
    pool: PgPool,
    statements: ProviderStateStatements,
}

impl PostgresProviderState {
    #[must_use]
    pub fn create(pool: PgPool) -> Self {
        Self {
            pool,
            statements: ProviderStateStatements::prepare(),
        }
    }
}

#[async_trait]
impl StoresProviderState for PostgresProviderState {
    async fn decide_pending_authorization(
        &self,
        id: Uuid,
        PendingDecision { subject, verdict }: PendingDecision,
    ) -> Result<DecidedAuthorization, ProviderStateError> {
        let approved_code = match verdict {
            PendingVerdict::Approved { code } => Some(code.as_bytes().to_vec()),
            PendingVerdict::Denied => None,
        };

        query_as::<_, DecidedRow>(self.statements.decide_pending_authorization.clone())
            .bind(id)
            .bind(subject.to_string())
            .bind(approved_code)
            .bind(AUTHORIZATION_CODE_LIFETIME.as_secs_f64())
            .fetch_optional(&self.pool)
            .await
            .map_err(|source| ProviderStateError::DecidePendingAuthorization {
                source: Box::new(source),
            })
            .and_then(|decided| match decided {
                Some(DecidedRow {
                    pending_document,
                    subject_matches: true,
                }) => serde_json::from_str::<PendingAuthorization>(&pending_document)
                    .map_err(|source| ProviderStateError::DecidePendingAuthorization {
                        source: Box::new(source),
                    })
                    .map(|pending| match verdict {
                        PendingVerdict::Approved { .. } => {
                            DecidedAuthorization::Approved(Box::new(pending))
                        }
                        PendingVerdict::Denied => DecidedAuthorization::Denied(Box::new(pending)),
                    }),
                Some(DecidedRow {
                    subject_matches: false,
                    ..
                })
                | None => Ok(DecidedAuthorization::Unknown),
            })
    }

    async fn hold_pending_authorization(
        &self,
        id: Uuid,
        pending: PendingAuthorization,
    ) -> Result<(), ProviderStateError> {
        query(self.statements.hold_pending_authorization.clone())
            .bind(id)
            .bind(Json(pending))
            .bind(PENDING_AUTHORIZATION_LIFETIME.as_secs_f64())
            .execute(&self.pool)
            .await
            .map_err(|source| ProviderStateError::HoldPendingAuthorization {
                source: Box::new(source),
            })
            .map(|_| ())
    }

    async fn issue_code(
        &self,
        code: TokenDigest,
        grant: AuthorizationGrant,
    ) -> Result<(), ProviderStateError> {
        query(self.statements.issue_code.clone())
            .bind(code.as_bytes().to_vec())
            .bind(Json(grant))
            .bind(AUTHORIZATION_CODE_LIFETIME.as_secs_f64())
            .execute(&self.pool)
            .await
            .map_err(|source| ProviderStateError::IssueCode {
                source: Box::new(source),
            })
            .map(|_| ())
    }

    async fn redeem_code(
        &self,
        code: TokenDigest,
        CodeRedemptionRequest {
            admission,
            family,
            refresh,
        }: CodeRedemptionRequest<'_>,
    ) -> Result<CodeRedemption, ProviderStateError> {
        let refresh_token = match refresh {
            RefreshIssuance::Opened(token) => Some(token.as_bytes().to_vec()),
            RefreshIssuance::Withheld => None,
        };

        query_as::<_, RedeemedRow>(self.statements.redeem_code.clone())
            .bind(code.as_bytes().to_vec())
            .bind(admission.client_id.as_str())
            .bind(admission.redirect_uri.as_str())
            .bind(admission.code_challenge)
            .bind(family)
            .bind(refresh_token)
            .bind(REFRESH_FAMILY_LIFETIME.as_secs_f64())
            .fetch_optional(&self.pool)
            .await
            .map_err(|source| ProviderStateError::RedeemCode {
                source: Box::new(source),
            })
            .and_then(|redeemed| match redeemed {
                Some(RedeemedRow { replayed: true, .. }) => Ok(CodeRedemption::Replayed),
                Some(RedeemedRow {
                    admitted: true,
                    grant_document,
                    ..
                }) => serde_json::from_str::<AuthorizationGrant>(&grant_document)
                    .map_err(|source| ProviderStateError::RedeemCode {
                        source: Box::new(source),
                    })
                    .map(|grant| CodeRedemption::Redeemed(Box::new(grant))),
                Some(RedeemedRow {
                    admitted: false, ..
                }) => Ok(CodeRedemption::Refused),
                None => Ok(CodeRedemption::Unknown),
            })
    }

    async fn revoke_refresh_token(
        &self,
        presented: TokenDigest,
        client_id: &ClientId,
    ) -> Result<RefreshRevocation, ProviderStateError> {
        query_scalar::<_, bool>(self.statements.revoke_refresh_token.clone())
            .bind(presented.as_bytes().to_vec())
            .bind(client_id.as_str())
            .fetch_optional(&self.pool)
            .await
            .map_err(|source| ProviderStateError::RevokeRefreshToken {
                source: Box::new(source),
            })
            .map(|client_matches| match client_matches {
                Some(true) => RefreshRevocation::Revoked,
                Some(false) => RefreshRevocation::ForeignClient,
                None => RefreshRevocation::Unknown,
            })
    }

    async fn rotate_refresh_token(
        &self,
        presented: TokenDigest,
        next: TokenDigest,
        admission: RefreshAdmission<'_>,
    ) -> Result<RefreshRotation, ProviderStateError> {
        let narrowed = match admission.scope {
            RefreshScope::Granted => None,
            RefreshScope::Narrowed(scopes) => Some(Json(scopes)),
        };

        query_as::<_, RotatedRow>(self.statements.rotate_refresh_token.clone())
            .bind(presented.as_bytes().to_vec())
            .bind(next.as_bytes().to_vec())
            .bind(admission.client_id.as_str())
            .bind(narrowed)
            .fetch_optional(&self.pool)
            .await
            .map_err(|source| ProviderStateError::RotateRefreshToken {
                source: Box::new(source),
            })
            .and_then(|rotated| match rotated {
                Some(RotatedRow {
                    superseded: true, ..
                }) => Ok(RefreshRotation::Replayed),
                Some(RotatedRow {
                    client_matches: true,
                    family,
                    grant_document: Some(grant_document),
                    scope_within: true,
                    ..
                }) => serde_json::from_str::<AuthorizationGrant>(&grant_document)
                    .map_err(|source| ProviderStateError::RotateRefreshToken {
                        source: Box::new(source),
                    })
                    .map(|grant| RefreshRotation::Rotated(refresh_family_of(family, grant))),
                Some(RotatedRow {
                    client_matches: false,
                    grant_document: Some(_),
                    ..
                }) => Ok(RefreshRotation::ForeignClient),
                Some(RotatedRow {
                    grant_document: Some(_),
                    ..
                }) => Ok(RefreshRotation::ScopeExceeded),
                Some(RotatedRow {
                    grant_document: None,
                    ..
                })
                | None => Ok(RefreshRotation::Unknown),
            })
    }
}
