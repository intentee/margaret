use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use margaret_issuer_directory::issuer_directory::IssuerDirectory;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

pub struct PolledDirectory {
    cancellation_token: CancellationToken,
    run: JoinHandle<()>,
}

impl PolledDirectory {
    /// # Panics
    ///
    /// Panics when the trusted issuers do not form a directory.
    #[must_use]
    pub fn start(
        trusted_issuers: Vec<Arc<TrustedIssuer>>,
        request_client: IssuerRequestClient,
    ) -> Self {
        let directory = IssuerDirectory::create(Arc::new(request_client), trusted_issuers)
            .expect("the trusted issuers form a directory");
        let cancellation_token = CancellationToken::new();
        let run_token = cancellation_token.clone();
        let run = tokio::spawn(async move { directory.run(run_token).await });

        Self {
            cancellation_token,
            run,
        }
    }

    /// # Panics
    ///
    /// Panics when the directory task does not join.
    pub async fn stop(self) {
        self.cancellation_token.cancel();
        self.run.await.expect("the directory task joins");
    }
}
