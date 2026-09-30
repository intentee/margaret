use std::ops::ControlFlow;
use std::sync::Arc;

use async_trait::async_trait;
use moka::future::Cache;
use moka::ops::compute::Op;
use uuid::Uuid;

use margaret_oauth_vocabulary::client_id::ClientId;

use crate::authorization_code_lifetime::AUTHORIZATION_CODE_LIFETIME;
use crate::authorization_grant::AuthorizationGrant;
use crate::code_redemption::CodeRedemption;
use crate::code_redemption_request::CodeRedemptionRequest;
use crate::code_state::CodeState;
use crate::decided_authorization::DecidedAuthorization;
use crate::pending_authorization::PendingAuthorization;
use crate::pending_authorization_lifetime::PENDING_AUTHORIZATION_LIFETIME;
use crate::pending_decision::PendingDecision;
use crate::pending_verdict::PendingVerdict;
use crate::provider_state_error::ProviderStateError;
use crate::refresh_admission::RefreshAdmission;
use crate::refresh_family::RefreshFamily;
use crate::refresh_family_lifetime::REFRESH_FAMILY_LIFETIME;
use crate::refresh_issuance::RefreshIssuance;
use crate::refresh_revocation::RefreshRevocation;
use crate::refresh_rotation::RefreshRotation;
use crate::refresh_token_state::RefreshTokenState;
use crate::stores_provider_state::StoresProviderState;
use crate::token_digest::TokenDigest;

pub struct MemoryProviderState {
    codes: Cache<TokenDigest, CodeState>,
    families: Cache<Uuid, RefreshFamily>,
    pending: Cache<Uuid, PendingAuthorization>,
    tokens: Cache<TokenDigest, RefreshTokenState>,
}

impl MemoryProviderState {
    #[must_use]
    pub fn create() -> Self {
        Self {
            codes: Cache::builder()
                .time_to_live(AUTHORIZATION_CODE_LIFETIME)
                .build(),
            families: Cache::builder()
                .time_to_live(REFRESH_FAMILY_LIFETIME)
                .build(),
            pending: Cache::builder()
                .time_to_live(PENDING_AUTHORIZATION_LIFETIME)
                .build(),
            tokens: Cache::builder()
                .time_to_live(REFRESH_FAMILY_LIFETIME)
                .build(),
        }
    }

    async fn open_family(&self, grant: &AuthorizationGrant, id: Uuid, token: TokenDigest) {
        self.families
            .insert(
                id,
                RefreshFamily {
                    auth_time: grant.auth_time,
                    client_id: grant.client_id.clone(),
                    id,
                    scopes: grant.scopes.clone(),
                    subject: grant.subject,
                },
            )
            .await;
        self.tokens
            .insert(token, RefreshTokenState::Current { family: id })
            .await;
    }
}

#[async_trait]
impl StoresProviderState for MemoryProviderState {
    async fn decide_pending_authorization(
        &self,
        id: Uuid,
        PendingDecision { subject, verdict }: PendingDecision,
    ) -> Result<DecidedAuthorization, ProviderStateError> {
        Ok(match self.pending.remove(&id).await {
            Some(pending) if pending.grant.subject == subject => match verdict {
                PendingVerdict::Approved { code } => {
                    self.codes
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
        self.pending.insert(id, pending).await;

        Ok(())
    }

    async fn issue_code(
        &self,
        code: TokenDigest,
        grant: AuthorizationGrant,
    ) -> Result<(), ProviderStateError> {
        self.codes
            .insert(code, CodeState::Issued(Arc::new(grant)))
            .await;

        Ok(())
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
        let mut redemption = CodeRedemption::Unknown;

        self.codes
            .entry(code)
            .and_compute_with(|existing| async {
                match existing.map(moka::Entry::into_value) {
                    Some(CodeState::Issued(grant)) => {
                        redemption = if admission.admits(&grant) {
                            CodeRedemption::Redeemed(Box::new(grant.as_ref().clone()))
                        } else {
                            CodeRedemption::Refused
                        };

                        Op::Put(CodeState::Redeemed { family })
                    }
                    Some(CodeState::Redeemed { family }) => {
                        self.families.invalidate(&family).await;
                        redemption = CodeRedemption::Replayed;

                        Op::Nop
                    }
                    None => Op::Nop,
                }
            })
            .await;

        if let CodeRedemption::Redeemed(grant) = &redemption
            && let RefreshIssuance::Opened(token) = refresh
        {
            self.open_family(grant, family, token).await;
        }

        Ok(redemption)
    }

    async fn revoke_refresh_token(
        &self,
        presented: TokenDigest,
        client_id: &ClientId,
    ) -> Result<RefreshRevocation, ProviderStateError> {
        let Some(RefreshTokenState::Current { family }) = self.tokens.get(&presented).await else {
            return Ok(RefreshRevocation::Unknown);
        };

        Ok(match self.families.get(&family).await {
            Some(live) if live.client_id == *client_id => {
                self.families.invalidate(&family).await;

                RefreshRevocation::Revoked
            }
            Some(_) => RefreshRevocation::ForeignClient,
            None => RefreshRevocation::Unknown,
        })
    }

    async fn rotate_refresh_token(
        &self,
        presented: TokenDigest,
        next: TokenDigest,
        admission: RefreshAdmission<'_>,
    ) -> Result<RefreshRotation, ProviderStateError> {
        let mut rotation = RefreshRotation::Unknown;

        self.tokens
            .entry(presented)
            .and_compute_with(|existing| async {
                match existing.map(moka::Entry::into_value) {
                    Some(RefreshTokenState::Current { family }) => {
                        match self.families.get(&family).await {
                            Some(live) => match admission.admits(&live) {
                                ControlFlow::Break(refusal) => {
                                    rotation = refusal;

                                    Op::Nop
                                }
                                ControlFlow::Continue(()) => {
                                    rotation = RefreshRotation::Rotated(live);

                                    Op::Put(RefreshTokenState::Superseded { family })
                                }
                            },
                            None => Op::Nop,
                        }
                    }
                    Some(RefreshTokenState::Superseded { family }) => {
                        self.families.invalidate(&family).await;
                        rotation = RefreshRotation::Replayed;

                        Op::Nop
                    }
                    None => Op::Nop,
                }
            })
            .await;

        if let RefreshRotation::Rotated(family) = &rotation {
            self.tokens
                .insert(next, RefreshTokenState::Current { family: family.id })
                .await;
        }

        Ok(rotation)
    }
}
