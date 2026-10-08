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
    /// Resolves the token to its family while the family is open, reporting a token that a
    /// rotation replaced as superseded.
    ///
    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn find_refresh_token(&self, token: TokenDigest) -> anyhow::Result<RefreshTokenLookup>;

    /// Holds the pending authorization until a single take removes it.
    ///
    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn hold_pending_authorization(
        &self,
        id: Uuid,
        pending: PendingAuthorization,
    ) -> anyhow::Result<()>;

    /// Holds the issued code until its redemption.
    ///
    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn issue_code(&self, code: TokenDigest, issued: IssuedCode) -> anyhow::Result<()>;

    /// Opens the family with its first token, atomically across every instance, unless the family
    /// was revoked before, which it reports as revoked.
    ///
    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn open_refresh_family(
        &self,
        family: Uuid,
        record: RefreshFamily,
        first_token: TokenDigest,
    ) -> anyhow::Result<FamilyOpening>;

    /// Redeems the code for the family exactly once across every instance, reporting every later
    /// redemption with the family of the first one.
    ///
    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn redeem_code(&self, code: TokenDigest, family: Uuid) -> anyhow::Result<CodeRedemption>;

    /// Revokes the family for good, including a family that is not opened yet.
    ///
    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn revoke_refresh_family(&self, family: Uuid) -> anyhow::Result<()>;

    /// Replaces the presented current token with the next one exactly once across every instance,
    /// reporting every other rotation of the same token as superseded.
    ///
    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn rotate_refresh_token(
        &self,
        presented: TokenDigest,
        next: TokenDigest,
    ) -> anyhow::Result<RefreshRotation>;

    /// Removes and returns the pending authorization exactly once across every instance.
    ///
    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn take_pending_authorization(
        &self,
        id: Uuid,
    ) -> anyhow::Result<PendingAuthorizationTake>;
}
