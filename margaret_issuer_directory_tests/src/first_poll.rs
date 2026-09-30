use std::sync::Arc;

use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::polled_directory::PolledDirectory;

pub async fn first_poll(
    trusted_issuer: &Arc<TrustedIssuer>,
    request_client: IssuerRequestClient,
) -> KeySetRefresh {
    let snapshot = trusted_issuer.key_set.snapshot();
    let directory = PolledDirectory::start(vec![Arc::clone(trusted_issuer)], request_client);
    let refresh = trusted_issuer.key_set.refreshed_since(&snapshot).await;

    directory.stop().await;

    refresh
}
