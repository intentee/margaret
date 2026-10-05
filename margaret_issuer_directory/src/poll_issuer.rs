use std::sync::Arc;

use log::debug;
use log::error;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;

use crate::await_next_fetch::await_next_fetch;
use crate::fetch_key_set::fetch_key_set;
use crate::issuer_group::IssuerGroup;
use crate::key_set_poll::KeySetPoll;
use crate::next_fetch::NextFetch;

pub(crate) async fn poll_issuer(
    client: &IssuerRequestClient,
    group: &IssuerGroup,
    cancellation_token: &CancellationToken,
) {
    let issuer = group.issuer();

    loop {
        let started_at = Instant::now();

        group.start_fetch();

        match fetch_key_set(client, group, cancellation_token).await {
            KeySetPoll::Cancelled => break,
            KeySetPoll::Failed(failure) => {
                error!("Unable to poll the key set of the issuer '{issuer}': {failure}");
                group.fail_fetch();
            }
            KeySetPoll::Fetched(AcceptedKeySetDocument {
                disclosed_keys,
                ignored_keys,
                key_set,
            }) => {
                for disclosed_key in disclosed_keys {
                    error!("Distrusting a key of the issuer '{issuer}': {disclosed_key}");
                }

                for ignored_key in ignored_keys {
                    debug!("Ignoring a key of the issuer '{issuer}': {ignored_key}");
                }

                group.hold(&Arc::new(key_set));
            }
        }

        let due_at = group.next_fetch_due(started_at);

        if let NextFetch::Cancelled =
            await_next_fetch(group, started_at, due_at, cancellation_token).await
        {
            break;
        }
    }

    group.stop_polling();
}
