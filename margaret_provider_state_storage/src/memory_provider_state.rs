use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_token_digest::token_digest::TokenDigest;

use crate::authorization_grant::AuthorizationGrant;
use crate::code_spending::CodeSpending;
use crate::code_state::CodeState;
use crate::decided_authorization::DecidedAuthorization;
use crate::live_refresh_token::LiveRefreshToken;
use crate::pending_authorization::PendingAuthorization;
use crate::pending_decision::PendingDecision;
use crate::pending_verdict::PendingVerdict;
use crate::presented_code::PresentedCode;
use crate::presented_refresh_token::PresentedRefreshToken;
use crate::provider_state_caches::ProviderStateCaches;
use crate::provider_state_error::ProviderStateError;
use crate::refresh_family::RefreshFamily;
use crate::refresh_issuance::RefreshIssuance;
use crate::refresh_revocation::RefreshRevocation;
use crate::refresh_rotation::RefreshRotation;
use crate::refresh_token_standing::RefreshTokenStanding;
use crate::refresh_token_state::RefreshTokenState;
use crate::stores_provider_state::StoresProviderState;

pub struct MemoryProviderState {
    caches: Mutex<ProviderStateCaches>,
}

impl MemoryProviderState {
    #[must_use]
    pub fn create() -> Self {
        Self {
            caches: Mutex::new(ProviderStateCaches::create()),
        }
    }
}

#[async_trait]
impl StoresProviderState for MemoryProviderState {
    async fn decide_pending_authorization(
        &self,
        id: Uuid,
        PendingDecision { subject, verdict }: PendingDecision,
    ) -> Result<DecidedAuthorization, ProviderStateError> {
        let caches = self.caches.lock().await;

        Ok(match caches.pending.remove(&id).await {
            Some(pending) if pending.grant.subject == subject => match verdict {
                PendingVerdict::Approved { code } => {
                    caches
                        .codes
                        .insert(code, CodeState::Issued(Arc::new(pending.grant.clone())))
                        .await;

                    DecidedAuthorization::Approved(Box::new(pending))
                }
                PendingVerdict::Denied => DecidedAuthorization::Denied(Box::new(pending)),
            },
            Some(_) | None => DecidedAuthorization::Unknown,
        })
    }

    async fn hold_pending_authorization(
        &self,
        id: Uuid,
        pending: PendingAuthorization,
    ) -> Result<(), ProviderStateError> {
        self.caches.lock().await.pending.insert(id, pending).await;

        Ok(())
    }

    async fn issue_code(
        &self,
        code: TokenDigest,
        grant: AuthorizationGrant,
    ) -> Result<(), ProviderStateError> {
        self.caches
            .lock()
            .await
            .codes
            .insert(code, CodeState::Issued(Arc::new(grant)))
            .await;

        Ok(())
    }

    async fn present_code(&self, code: TokenDigest) -> Result<PresentedCode, ProviderStateError> {
        let caches = self.caches.lock().await;

        Ok(match caches.codes.get(&code).await {
            Some(CodeState::Issued(grant)) => {
                PresentedCode::Issued(Box::new(AuthorizationGrant::clone(&grant)))
            }
            Some(CodeState::Spent { family }) => {
                caches.families.invalidate(&family).await;

                PresentedCode::Replayed
            }
            None => PresentedCode::Unknown,
        })
    }

    async fn present_refresh_token(
        &self,
        presented: TokenDigest,
    ) -> Result<PresentedRefreshToken, ProviderStateError> {
        let caches = self.caches.lock().await;

        Ok(match caches.live_refresh_token(&presented).await {
            Some(LiveRefreshToken {
                family,
                state:
                    RefreshTokenState {
                        standing: RefreshTokenStanding::Current,
                        ..
                    },
            }) => PresentedRefreshToken::Current(family),
            Some(LiveRefreshToken {
                state:
                    RefreshTokenState {
                        family,
                        standing: RefreshTokenStanding::Superseded,
                    },
                ..
            }) => {
                caches.families.invalidate(&family).await;

                PresentedRefreshToken::Replayed
            }
            None => PresentedRefreshToken::Unknown,
        })
    }

    async fn revoke_refresh_token(
        &self,
        presented: TokenDigest,
        client_id: &ClientId,
    ) -> Result<RefreshRevocation, ProviderStateError> {
        let caches = self.caches.lock().await;

        Ok(match caches.live_refresh_token(&presented).await {
            Some(LiveRefreshToken {
                family: RefreshFamily {
                    client_id: owner, ..
                },
                state:
                    RefreshTokenState {
                        family,
                        standing: RefreshTokenStanding::Current,
                    },
            }) if owner == *client_id => {
                caches.families.invalidate(&family).await;

                RefreshRevocation::Revoked
            }
            Some(LiveRefreshToken {
                state:
                    RefreshTokenState {
                        standing: RefreshTokenStanding::Current,
                        ..
                    },
                ..
            }) => RefreshRevocation::ForeignClient,
            Some(LiveRefreshToken {
                state:
                    RefreshTokenState {
                        standing: RefreshTokenStanding::Superseded,
                        ..
                    },
                ..
            })
            | None => RefreshRevocation::Unknown,
        })
    }

    async fn rotate_refresh_token(
        &self,
        presented: TokenDigest,
        next: TokenDigest,
    ) -> Result<RefreshRotation, ProviderStateError> {
        let caches = self.caches.lock().await;

        Ok(match caches.live_refresh_token(&presented).await {
            Some(LiveRefreshToken {
                state:
                    RefreshTokenState {
                        family,
                        standing: RefreshTokenStanding::Current,
                    },
                ..
            }) => {
                caches
                    .tokens
                    .insert(
                        presented,
                        RefreshTokenState {
                            family,
                            standing: RefreshTokenStanding::Superseded,
                        },
                    )
                    .await;
                caches
                    .tokens
                    .insert(
                        next,
                        RefreshTokenState {
                            family,
                            standing: RefreshTokenStanding::Current,
                        },
                    )
                    .await;

                RefreshRotation::Rotated
            }
            Some(LiveRefreshToken {
                state:
                    RefreshTokenState {
                        family,
                        standing: RefreshTokenStanding::Superseded,
                    },
                ..
            }) => {
                caches.families.invalidate(&family).await;

                RefreshRotation::Replayed
            }
            None => RefreshRotation::Revoked,
        })
    }

    async fn spend_code(
        &self,
        code: TokenDigest,
        refresh: RefreshIssuance,
    ) -> Result<CodeSpending, ProviderStateError> {
        let caches = self.caches.lock().await;

        Ok(match caches.codes.get(&code).await {
            Some(CodeState::Issued(grant)) => {
                let family = Uuid::new_v4();

                caches.codes.insert(code, CodeState::Spent { family }).await;

                match refresh {
                    RefreshIssuance::Opened(token) => {
                        caches
                            .families
                            .insert(family, RefreshFamily::opened_by(&grant))
                            .await;
                        caches
                            .tokens
                            .insert(
                                token,
                                RefreshTokenState {
                                    family,
                                    standing: RefreshTokenStanding::Current,
                                },
                            )
                            .await;
                    }
                    RefreshIssuance::Withheld => {}
                }

                CodeSpending::Spent
            }
            Some(CodeState::Spent { family }) => {
                caches.families.invalidate(&family).await;

                CodeSpending::Replayed
            }
            None => CodeSpending::Expired,
        })
    }
}
