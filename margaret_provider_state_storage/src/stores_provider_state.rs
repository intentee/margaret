use async_trait::async_trait;
use uuid::Uuid;

use margaret_oauth_vocabulary::client_id::ClientId;

use crate::authorization_grant::AuthorizationGrant;
use crate::code_redemption::CodeRedemption;
use crate::code_redemption_request::CodeRedemptionRequest;
use crate::decided_authorization::DecidedAuthorization;
use crate::pending_authorization::PendingAuthorization;
use crate::pending_decision::PendingDecision;
use crate::provider_state_error::ProviderStateError;
use crate::refresh_admission::RefreshAdmission;
use crate::refresh_revocation::RefreshRevocation;
use crate::refresh_rotation::RefreshRotation;
use crate::token_digest::TokenDigest;

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
    /// Returns `ProviderStateError` when the backend cannot redeem the code.
    async fn redeem_code(
        &self,
        code: TokenDigest,
        request: CodeRedemptionRequest<'_>,
    ) -> Result<CodeRedemption, ProviderStateError>;

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
        admission: RefreshAdmission<'_>,
    ) -> Result<RefreshRotation, ProviderStateError>;
}
