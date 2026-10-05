use async_trait::async_trait;
use uuid::Uuid;

use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_token_digest::token_digest::TokenDigest;

use crate::authorization_grant::AuthorizationGrant;
use crate::code_spending::CodeSpending;
use crate::decided_authorization::DecidedAuthorization;
use crate::pending_authorization::PendingAuthorization;
use crate::pending_decision::PendingDecision;
use crate::presented_code::PresentedCode;
use crate::presented_refresh_token::PresentedRefreshToken;
use crate::provider_state_error::ProviderStateError;
use crate::refresh_issuance::RefreshIssuance;
use crate::refresh_revocation::RefreshRevocation;
use crate::refresh_rotation::RefreshRotation;

#[async_trait]
pub trait StoresProviderState: Send + Sync {
    /// # Errors
    ///
    /// Returns `ProviderStateError` when the backend cannot decide the pending authorization.
    async fn decide_pending_authorization(
        &self,
        id: Uuid,
        decision: PendingDecision,
    ) -> Result<DecidedAuthorization, ProviderStateError>;

    /// # Errors
    ///
    /// Returns `ProviderStateError` when the backend cannot hold the pending authorization.
    async fn hold_pending_authorization(
        &self,
        id: Uuid,
        pending: PendingAuthorization,
    ) -> Result<(), ProviderStateError>;

    /// # Errors
    ///
    /// Returns `ProviderStateError` when the backend cannot store the code.
    async fn issue_code(
        &self,
        code: TokenDigest,
        grant: AuthorizationGrant,
    ) -> Result<(), ProviderStateError>;

    /// # Errors
    ///
    /// Returns `ProviderStateError` when the backend cannot look up the code or revoke the
    /// refresh family of a replayed one.
    async fn present_code(&self, code: TokenDigest) -> Result<PresentedCode, ProviderStateError>;

    /// # Errors
    ///
    /// Returns `ProviderStateError` when the backend cannot look up the refresh token or
    /// revoke the family of a replayed one.
    async fn present_refresh_token(
        &self,
        presented: TokenDigest,
    ) -> Result<PresentedRefreshToken, ProviderStateError>;

    /// # Errors
    ///
    /// Returns `ProviderStateError` when the backend cannot revoke the refresh token.
    async fn revoke_refresh_token(
        &self,
        presented: TokenDigest,
        client_id: &ClientId,
    ) -> Result<RefreshRevocation, ProviderStateError>;

    /// # Errors
    ///
    /// Returns `ProviderStateError` when the backend cannot rotate the refresh token.
    async fn rotate_refresh_token(
        &self,
        presented: TokenDigest,
        next: TokenDigest,
    ) -> Result<RefreshRotation, ProviderStateError>;

    /// # Errors
    ///
    /// Returns `ProviderStateError` when the backend cannot spend the code.
    async fn spend_code(
        &self,
        code: TokenDigest,
        refresh: RefreshIssuance,
    ) -> Result<CodeSpending, ProviderStateError>;
}
