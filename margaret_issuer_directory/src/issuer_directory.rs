use std::sync::Arc;

use futures_util::future::join_all;
use tokio_util::sync::CancellationToken;

use margaret_issuer_request::issuer_request_client::IssuerRequestClient;

use crate::poll_key_set::poll_key_set;
use crate::polled_key_set::PolledKeySet;

pub struct IssuerDirectory {
    key_sets: Vec<Arc<PolledKeySet>>,
    request_client: Arc<IssuerRequestClient>,
}

impl IssuerDirectory {
    #[must_use]
    pub fn create(
        request_client: Arc<IssuerRequestClient>,
        key_sets: Vec<Arc<PolledKeySet>>,
    ) -> Self {
        Self {
            key_sets,
            request_client,
        }
    }

    pub async fn run(&self, cancellation_token: CancellationToken) {
        join_all(
            self.key_sets
                .iter()
                .map(|polled| poll_key_set(&self.request_client, polled, &cancellation_token)),
        )
        .await;
    }
}
