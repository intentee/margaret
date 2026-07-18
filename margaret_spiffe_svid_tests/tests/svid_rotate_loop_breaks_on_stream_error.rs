use std::time::Duration;

use anyhow::anyhow;
use futures_util::stream;
use spiffe::X509Context;
use tokio::sync::broadcast;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

use margaret_spiffe_svid::svid_rotate_loop::svid_rotate_loop;

#[tokio::test]
async fn breaks_on_stream_error() {
    let (x509_context_tx, _x509_context_rx) = broadcast::channel::<X509Context>(1);

    let completion = timeout(
        Duration::from_secs(1),
        svid_rotate_loop(
            Box::pin(stream::iter(vec![Err(anyhow!("boom"))])),
            &x509_context_tx,
            CancellationToken::new(),
        ),
    )
    .await;

    assert!(completion.is_ok());
}
