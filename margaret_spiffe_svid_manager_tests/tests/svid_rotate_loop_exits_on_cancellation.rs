use std::time::Duration;

use futures_util::stream;
use spiffe::X509Context;
use tokio::sync::broadcast;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

use margaret_spiffe_svid_manager::svid_rotate_loop::svid_rotate_loop;

#[tokio::test]
async fn exits_on_cancellation() {
    let (x509_context_tx, _x509_context_rx) = broadcast::channel::<X509Context>(1);
    let cancellation_token = CancellationToken::new();
    let cancellation_token_for_loop = cancellation_token.clone();

    let loop_task = tokio::spawn(async move {
        svid_rotate_loop(
            Box::pin(stream::pending()),
            &x509_context_tx,
            cancellation_token_for_loop,
        )
        .await;
    });

    cancellation_token.cancel();

    timeout(Duration::from_secs(1), loop_task)
        .await
        .unwrap()
        .unwrap();
}
