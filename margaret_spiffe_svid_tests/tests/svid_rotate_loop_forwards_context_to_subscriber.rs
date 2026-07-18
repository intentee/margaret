use futures_util::stream;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

use margaret_spiffe_svid::svid_rotate_loop::svid_rotate_loop;
use margaret_spiffe_svid_tests::build_workload_x509_context::build_workload_x509_context;

#[tokio::test]
async fn forwards_context_to_subscriber() {
    let context = build_workload_x509_context().unwrap();
    let (x509_context_tx, mut x509_context_rx) = broadcast::channel(1);

    svid_rotate_loop(
        Box::pin(stream::iter(vec![Ok(context)])),
        &x509_context_tx,
        CancellationToken::new(),
    )
    .await;

    let received = x509_context_rx.try_recv().unwrap();

    assert!(received.default_svid().is_some());
}
