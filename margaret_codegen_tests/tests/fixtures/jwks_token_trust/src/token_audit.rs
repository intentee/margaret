use std::sync::Arc;

use chrono::DateTime;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use margaret::framework::identity_session::session_access_token_claims::SessionAccessTokenClaims;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;

use crate::margaret::jwks::JwksSecretStore;

#[service]
pub struct TokenAudit {
    secret_store: Arc<JwksSecretStore>,
}

impl TokenAudit {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(secret_store: Arc<JwksSecretStore>) -> anyhow::Result<Self> {
        Ok(Self { secret_store })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> anyhow::Result<()> {
        let _signing = self.secret_store.issue_session_access_token(
            &SessionAccessTokenClaims {
                auth_time: DateTime::UNIX_EPOCH,
                sid: Uuid::nil(),
                sub: Uuid::nil(),
            },
            "audit",
            DateTime::UNIX_EPOCH,
        );

        cancellation_token.cancelled().await;

        Ok(())
    }
}
