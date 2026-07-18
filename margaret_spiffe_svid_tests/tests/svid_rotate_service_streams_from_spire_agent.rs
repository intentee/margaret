use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use trzcina::Service as _;

use margaret_spiffe_svid::svid_rotate_service::SvidRotateService;
use margaret_spiffe_svid_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_tests::spire_test_cluster_params::SpireTestClusterParams;

#[tokio::test]
async fn streams_x509_context_from_running_spire_agent() {
    let cluster = SpireTestCluster::start(
        tempfile::tempdir().unwrap(),
        SpireTestClusterParams::default(),
        20015,
    )
    .await
    .unwrap();

    let (x509_context_tx, mut x509_context_rx) = broadcast::channel(1);
    let service = SvidRotateService {
        spire_agent_addr: format!("unix://{}", cluster.agent_socket_path().display()),
        x509_context_tx,
    };

    let cancellation_token = CancellationToken::new();
    let cancellation_token_for_service = cancellation_token.clone();
    let service_task =
        tokio::spawn(async move { Box::new(service).run(cancellation_token_for_service).await });

    let received = x509_context_rx.recv().await.unwrap();
    assert!(received.default_svid().is_some());

    cancellation_token.cancel();
    service_task.await.unwrap().unwrap();
}
