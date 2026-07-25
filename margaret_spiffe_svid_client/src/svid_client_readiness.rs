use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_sync_holder::sync_holder_presence::SyncHolderPresence;
use margaret_sync_holder::sync_holder_subscription::SyncHolderSubscription;

use crate::svid_server_cert_verifier::SvidServerCertVerifier;

pub struct SvidClientReadiness {
    server_cert_verifier_subscription: SyncHolderSubscription<Arc<SvidServerCertVerifier>>,
}

impl SvidClientReadiness {
    #[must_use]
    pub fn new(
        server_cert_verifier_subscription: SyncHolderSubscription<Arc<SvidServerCertVerifier>>,
    ) -> Self {
        Self {
            server_cert_verifier_subscription,
        }
    }

    pub async fn wait_until_ready(
        &mut self,
        cancellation_token: &CancellationToken,
    ) -> SyncHolderPresence {
        self.server_cert_verifier_subscription
            .wait_until_present(cancellation_token)
            .await
    }
}

#[cfg(test)]
mod tests {
    use tokio_util::sync::CancellationToken;

    use margaret_sync_holder::sync_holder_presence::SyncHolderPresence;

    use crate::svid_server_cert_verifier_facade::SvidServerCertVerifierFacade;

    use super::SvidClientReadiness;

    #[tokio::test]
    async fn wait_until_ready_reports_cancellation_before_the_verifier_arrives() {
        let facade = SvidServerCertVerifierFacade::default();
        let mut readiness = SvidClientReadiness::new(facade.subscribe());
        let cancellation_token = CancellationToken::new();

        cancellation_token.cancel();

        assert_eq!(
            readiness.wait_until_ready(&cancellation_token).await,
            SyncHolderPresence::Cancelled
        );
    }
}
