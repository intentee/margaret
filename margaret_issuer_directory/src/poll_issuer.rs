use log::debug;
use log::error;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::await_next_fetch::await_next_fetch;
use crate::fetch_key_set::fetch_key_set;
use crate::key_set_poll::KeySetPoll;
use crate::next_fetch::NextFetch;

pub(crate) async fn poll_issuer(
    client: &IssuerRequestClient,
    trusted_issuer: &TrustedIssuer,
    cancellation_token: &CancellationToken,
) {
    let issuer = &trusted_issuer.trust.token_trust().issuer;

    loop {
        let started_at = Instant::now();

        trusted_issuer.key_set.start_fetch();

        match fetch_key_set(client, trusted_issuer, cancellation_token).await {
            KeySetPoll::Cancelled => break,
            KeySetPoll::Failed(failure) => {
                error!("Unable to poll the key set of the issuer '{issuer}': {failure}");
                trusted_issuer.key_set.fail_fetch();
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

                trusted_issuer.key_set.hold(key_set);
            }
        }

        let due_at = trusted_issuer
            .key_set
            .snapshot()
            .holding
            .next_fetch_due(started_at);

        if let NextFetch::Cancelled = await_next_fetch(
            &trusted_issuer.key_set,
            started_at,
            due_at,
            cancellation_token,
        )
        .await
        {
            break;
        }
    }

    trusted_issuer.key_set.stop_polling();
}
