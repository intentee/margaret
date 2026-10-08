use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

use margaret::framework::authorization_grants::code_redemption::CodeRedemption;
use margaret::framework::authorization_grants::family_opening::FamilyOpening;
use margaret::framework::authorization_grants::issued_code::IssuedCode;
use margaret::framework::authorization_grants::pending_authorization::PendingAuthorization;
use margaret::framework::authorization_grants::pending_authorization_take::PendingAuthorizationTake;
use margaret::framework::authorization_grants::refresh_family::RefreshFamily;
use margaret::framework::authorization_grants::refresh_rotation::RefreshRotation;
use margaret::framework::authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret::framework::authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::macros::stores_authorization_grants;
use margaret::framework::token_digest::token_digest::TokenDigest;

use crate::stores::held_code::HeldCode;
use crate::stores::held_family::HeldFamily;
use crate::stores::held_grants::HeldGrants;
use crate::stores::held_refresh_token::HeldRefreshToken;
use crate::stores::refresh_token_standing::RefreshTokenStanding;

#[singleton]
#[stores_authorization_grants]
pub struct AuthorizationGrantStore {
    held: Mutex<HeldGrants>,
}

impl AuthorizationGrantStore {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self {
            held: Mutex::new(HeldGrants::create()),
        })
    }
}

#[async_trait]
impl StoresAuthorizationGrants for AuthorizationGrantStore {
    async fn find_refresh_token(&self, token: TokenDigest) -> anyhow::Result<RefreshTokenLookup> {
        Ok(self.held.lock().await.lookup(&token).await)
    }

    async fn hold_pending_authorization(
        &self,
        id: Uuid,
        pending: PendingAuthorization,
    ) -> anyhow::Result<()> {
        self.held.lock().await.pending.insert(id, pending).await;

        Ok(())
    }

    async fn issue_code(&self, code: TokenDigest, issued: IssuedCode) -> anyhow::Result<()> {
        self.held
            .lock()
            .await
            .codes
            .insert(code, HeldCode::Issued(Box::new(issued)))
            .await;

        Ok(())
    }

    async fn open_refresh_family(
        &self,
        family: Uuid,
        record: RefreshFamily,
        first_token: TokenDigest,
    ) -> anyhow::Result<FamilyOpening> {
        let held = self.held.lock().await;

        Ok(match held.families.get(&family).await {
            Some(HeldFamily::Revoked) => FamilyOpening::Revoked,
            Some(HeldFamily::Open(_)) | None => {
                held.families.insert(family, HeldFamily::Open(record)).await;
                held.refresh_tokens
                    .insert(
                        first_token,
                        HeldRefreshToken {
                            family,
                            standing: RefreshTokenStanding::Current,
                        },
                    )
                    .await;

                FamilyOpening::Opened
            }
        })
    }

    async fn redeem_code(&self, code: TokenDigest, family: Uuid) -> anyhow::Result<CodeRedemption> {
        let held = self.held.lock().await;

        Ok(match held.codes.get(&code).await {
            Some(HeldCode::Issued(issued)) => {
                held.codes.insert(code, HeldCode::Redeemed { family }).await;

                CodeRedemption::Redeemed(issued)
            }
            Some(HeldCode::Redeemed { family: redeemed }) => {
                CodeRedemption::AlreadyRedeemed { family: redeemed }
            }
            None => CodeRedemption::Unknown,
        })
    }

    async fn revoke_refresh_family(&self, family: Uuid) -> anyhow::Result<()> {
        self.held
            .lock()
            .await
            .families
            .insert(family, HeldFamily::Revoked)
            .await;

        Ok(())
    }

    async fn rotate_refresh_token(
        &self,
        presented: TokenDigest,
        next: TokenDigest,
    ) -> anyhow::Result<RefreshRotation> {
        let held = self.held.lock().await;

        Ok(match held.lookup(&presented).await {
            RefreshTokenLookup::Current { family, .. } => {
                held.refresh_tokens
                    .insert(
                        presented,
                        HeldRefreshToken {
                            family,
                            standing: RefreshTokenStanding::Superseded,
                        },
                    )
                    .await;
                held.refresh_tokens
                    .insert(
                        next,
                        HeldRefreshToken {
                            family,
                            standing: RefreshTokenStanding::Current,
                        },
                    )
                    .await;

                RefreshRotation::Rotated
            }
            RefreshTokenLookup::Superseded { family } => RefreshRotation::Superseded { family },
            RefreshTokenLookup::Unknown => RefreshRotation::Unknown,
        })
    }

    async fn take_pending_authorization(
        &self,
        id: Uuid,
    ) -> anyhow::Result<PendingAuthorizationTake> {
        Ok(match self.held.lock().await.pending.remove(&id).await {
            Some(pending) => PendingAuthorizationTake::Taken(Box::new(pending)),
            None => PendingAuthorizationTake::Absent,
        })
    }
}
