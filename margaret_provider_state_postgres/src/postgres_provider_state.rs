use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::query;
use sqlx::query_as;
use sqlx::query_scalar;
use sqlx::types::Json;
use uuid::Uuid;

use margaret_provider_state_storage::assertion_refusal::AssertionRefusal;
use margaret_provider_state_storage::assertion_retention::AssertionRetention;
use margaret_provider_state_storage::assertion_spending::AssertionSpending;
use margaret_provider_state_storage::authorization_code_lifetime::AUTHORIZATION_CODE_LIFETIME;
use margaret_provider_state_storage::authorization_grant::AuthorizationGrant;
use margaret_provider_state_storage::code_spending::CodeSpending;
use margaret_provider_state_storage::decided_authorization::DecidedAuthorization;
use margaret_provider_state_storage::pending_authorization::PendingAuthorization;
use margaret_provider_state_storage::pending_authorization_lifetime::PENDING_AUTHORIZATION_LIFETIME;
use margaret_provider_state_storage::pending_decision::PendingDecision;
use margaret_provider_state_storage::pending_verdict::PendingVerdict;
use margaret_provider_state_storage::presented_code::PresentedCode;
use margaret_provider_state_storage::presented_refresh_token::PresentedRefreshToken;
use margaret_provider_state_storage::provider_state_error::ProviderStateError;
use margaret_provider_state_storage::refresh_family::RefreshFamily;
use margaret_provider_state_storage::refresh_family_lifetime::REFRESH_FAMILY_LIFETIME;
use margaret_provider_state_storage::refresh_issuance::RefreshIssuance;
use margaret_provider_state_storage::refresh_revocation::RefreshRevocation;
use margaret_provider_state_storage::refresh_rotation::RefreshRotation;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

use crate::decided_row::DecidedRow;
use crate::presented_code_row::PresentedCodeRow;
use crate::presented_refresh_token_row::PresentedRefreshTokenRow;
use crate::provider_state_statements::ProviderStateStatements;
use crate::settlement_row::SettlementRow;

fn rotation_failure(source: sqlx::Error) -> ProviderStateError {
    ProviderStateError::RotateRefreshToken {
        source: Box::new(source),
    }
}

fn spending_failure(source: sqlx::Error) -> ProviderStateError {
    ProviderStateError::SpendCode {
        source: Box::new(source),
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

    async fn revoke_family(&self, family: Uuid) -> Result<(), sqlx::Error> {
        query(self.statements.revoke_refresh_family.clone())
            .bind(family)
            .execute(&self.pool)
            .await
            .map(|_| ())
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
            .bind(code.as_bytes().as_slice())
            .bind(Json(grant))
            .bind(AUTHORIZATION_CODE_LIFETIME.as_secs_f64())
            .execute(&self.pool)
            .await
            .map_err(|source| ProviderStateError::IssueCode {
                source: Box::new(source),
            })
            .map(|_| ())
    }

    async fn revoke_refresh_token(
        &self,
        presented: TokenDigest,
        client_id: &str,
    ) -> Result<RefreshRevocation, ProviderStateError> {
        query_scalar::<_, bool>(self.statements.revoke_refresh_token.clone())
            .bind(presented.as_bytes().as_slice())
            .bind(client_id)
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

    async fn present_code(&self, code: TokenDigest) -> Result<PresentedCode, ProviderStateError> {
        query_as::<_, PresentedCodeRow>(self.statements.present_code.clone())
            .bind(code.as_bytes().as_slice())
            .fetch_optional(&self.pool)
            .await
            .map_err(|source| ProviderStateError::PresentCode {
                source: Box::new(source),
            })
            .and_then(|presented| match presented {
                Some(PresentedCodeRow { spent: true, .. }) => Ok(PresentedCode::Replayed),
                Some(PresentedCodeRow {
                    grant_document,
                    spent: false,
                }) => serde_json::from_str::<AuthorizationGrant>(&grant_document)
                    .map_err(|source| ProviderStateError::PresentCode {
                        source: Box::new(source),
                    })
                    .map(|grant| PresentedCode::Issued(Box::new(grant))),
                None => Ok(PresentedCode::Unknown),
            })
    }

    async fn present_refresh_token(
        &self,
        presented: TokenDigest,
    ) -> Result<PresentedRefreshToken, ProviderStateError> {
        query_as::<_, PresentedRefreshTokenRow>(self.statements.present_refresh_token.clone())
            .bind(presented.as_bytes().as_slice())
            .fetch_optional(&self.pool)
            .await
            .map_err(|source| ProviderStateError::PresentRefreshToken {
                source: Box::new(source),
            })
            .and_then(|presented| match presented {
                Some(PresentedRefreshTokenRow {
                    superseded: true, ..
                }) => Ok(PresentedRefreshToken::Replayed),
                Some(PresentedRefreshTokenRow {
                    grant_document,
                    superseded: false,
                }) => serde_json::from_str::<AuthorizationGrant>(&grant_document)
                    .map_err(|source| ProviderStateError::PresentRefreshToken {
                        source: Box::new(source),
                    })
                    .map(|grant| PresentedRefreshToken::Current(RefreshFamily::opened_by(&grant))),
                None => Ok(PresentedRefreshToken::Unknown),
            })
    }

    async fn rotate_refresh_token(
        &self,
        presented: TokenDigest,
        next: TokenDigest,
    ) -> Result<RefreshRotation, ProviderStateError> {
        match query_as::<_, SettlementRow>(self.statements.rotate_refresh_token.clone())
            .bind(presented.as_bytes().as_slice())
            .bind(next.as_bytes().as_slice())
            .fetch_optional(&self.pool)
            .await
            .map_err(rotation_failure)?
        {
            Some(SettlementRow {
                family,
                replayed: true,
            }) => self
                .revoke_family(family)
                .await
                .map(|()| RefreshRotation::Replayed)
                .map_err(rotation_failure),
            Some(SettlementRow {
                replayed: false, ..
            }) => Ok(RefreshRotation::Rotated),
            None => Ok(RefreshRotation::Revoked),
        }
    }

    async fn spend_client_assertion(
        &self,
        client_id: &str,
        assertion: TokenDigest,
        expires_at: NumericDate,
        now: NumericDate,
    ) -> Result<AssertionSpending, ProviderStateError> {
        if let AssertionRetention::Expired = AssertionRetention::of(expires_at, now) {
            return Ok(AssertionSpending::Refused(AssertionRefusal::Expired));
        }

        query_scalar::<_, bool>(self.statements.spend_client_assertion.clone())
            .bind(client_id)
            .bind(assertion.as_bytes().as_slice())
            .bind(expires_at.seconds_since_epoch())
            .bind(now.seconds_since_epoch())
            .fetch_optional(&self.pool)
            .await
            .map_err(|source| ProviderStateError::SpendClientAssertion {
                source: Box::new(source),
            })
            .map(|spent| match spent {
                Some(_) => AssertionSpending::Spent,
                None => AssertionSpending::Refused(AssertionRefusal::Replayed),
            })
    }

    async fn spend_code(
        &self,
        code: TokenDigest,
        refresh: RefreshIssuance,
    ) -> Result<CodeSpending, ProviderStateError> {
        let refresh_token = match refresh {
            RefreshIssuance::Opened(token) => Some(token.as_bytes().to_vec()),
            RefreshIssuance::Withheld => None,
        };

        match query_as::<_, SettlementRow>(self.statements.spend_code.clone())
            .bind(code.as_bytes().as_slice())
            .bind(Uuid::new_v4())
            .bind(refresh_token)
            .bind(REFRESH_FAMILY_LIFETIME.as_secs_f64())
            .fetch_optional(&self.pool)
            .await
            .map_err(spending_failure)?
        {
            Some(SettlementRow {
                family,
                replayed: true,
            }) => self
                .revoke_family(family)
                .await
                .map(|()| CodeSpending::Replayed)
                .map_err(spending_failure),
            Some(SettlementRow {
                replayed: false, ..
            }) => Ok(CodeSpending::Spent),
            None => Ok(CodeSpending::Expired),
        }
    }
}
