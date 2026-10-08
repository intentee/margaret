use async_trait::async_trait;
use uuid::Uuid;

use margaret_token_digest::token_digest::TokenDigest;

use crate::code_redemption::CodeRedemption;
use crate::family_opening::FamilyOpening;
use crate::issued_code::IssuedCode;
use crate::pending_authorization::PendingAuthorization;
use crate::pending_authorization_take::PendingAuthorizationTake;
use crate::refresh_family::RefreshFamily;
use crate::refresh_rotation::RefreshRotation;
use crate::refresh_token_lookup::RefreshTokenLookup;

#[async_trait]
pub trait StoresAuthorizationGrants: Send + Sync {
    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn find_refresh_token(&self, token: TokenDigest) -> anyhow::Result<RefreshTokenLookup>;

    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn hold_pending_authorization(
        &self,
        id: Uuid,
        pending: PendingAuthorization,
    ) -> anyhow::Result<()>;

    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn issue_code(&self, code: TokenDigest, issued: IssuedCode) -> anyhow::Result<()>;

    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn open_refresh_family(
        &self,
        family: Uuid,
        record: RefreshFamily,
        first_token: TokenDigest,
    ) -> anyhow::Result<FamilyOpening>;

    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn redeem_code(&self, code: TokenDigest, family: Uuid) -> anyhow::Result<CodeRedemption>;

    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn revoke_refresh_family(&self, family: Uuid) -> anyhow::Result<()>;

    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn rotate_refresh_token(
        &self,
        presented: TokenDigest,
        next: TokenDigest,
    ) -> anyhow::Result<RefreshRotation>;

    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn take_pending_authorization(
        &self,
        id: Uuid,
    ) -> anyhow::Result<PendingAuthorizationTake>;
}
