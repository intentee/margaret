use futures_util::stream;
use spiffe::X509Context;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

use margaret_spiffe_svid_manager::svid_rotate_loop::svid_rotate_loop;
use margaret_spiffe_svid_manager_tests::build_workload_x509_context::build_workload_x509_context;

#[tokio::test]
async fn does_not_panic_when_no_subscribers() {
    let context = build_workload_x509_context().unwrap();
    let (x509_context_tx, _) = broadcast::channel::<X509Context>(1);

    svid_rotate_loop(
        Box::pin(stream::iter(vec![Ok(context)])),
        &x509_context_tx,
        CancellationToken::new(),
    )
    .await;
}
