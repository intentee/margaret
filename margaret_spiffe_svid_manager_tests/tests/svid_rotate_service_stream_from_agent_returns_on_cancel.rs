use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

use margaret_spiffe_svid_manager::svid_rotate_service::SvidRotateService;
use margaret_spiffe_svid_manager_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_manager_tests::spire_test_cluster_params::SpireTestClusterParams;

#[tokio::test]
async fn stream_from_agent_returns_when_cancelled_with_real_agent() {
    let cluster = SpireTestCluster::start(
        tempfile::tempdir().unwrap(),
        SpireTestClusterParams::default(),
        20014,
    )
    .await
    .unwrap();
    let (x509_context_tx, mut x509_context_rx) = broadcast::channel(1);
    let service = SvidRotateService {
        spire_agent_addr: format!("unix://{}", cluster.agent_socket_path().display()),
        x509_context_tx,
    };

    let cancellation_token = CancellationToken::new();
    let cancellation_token_for_task = cancellation_token.clone();
    let stream_task = tokio::spawn(async move {
        service.stream_from_agent(cancellation_token_for_task).await;
    });

    x509_context_rx.recv().await.unwrap();

    cancellation_token.cancel();
    stream_task.await.unwrap();
}
