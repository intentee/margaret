use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use margaret_issuer_directory::issuer_directory::IssuerDirectory;
use margaret_issuer_directory::polled_key_set::PolledKeySet;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;

pub struct PolledDirectory {
    cancellation_token: CancellationToken,
    run: JoinHandle<()>,
}

impl PolledDirectory {
    #[must_use]
    pub fn start(key_sets: Vec<Arc<PolledKeySet>>, request_client: IssuerRequestClient) -> Self {
        let directory = IssuerDirectory::create(Arc::new(request_client), key_sets);
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
