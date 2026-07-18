use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

use margaret_spiffe_svid::svid_rotate_service::SvidRotateService;
use margaret_spiffe_svid_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_tests::spire_test_cluster_params::SpireTestClusterParams;

#[tokio::test]
async fn stream_from_agent_returns_when_stream_rpc_is_rejected() {
    let cluster = SpireTestCluster::start(
        tempfile::tempdir().unwrap(),
        SpireTestClusterParams::default(),
        20013,
    )
    .await
    .unwrap();

    let (x509_context_tx, mut x509_context_rx) = broadcast::channel(1);
    let service = SvidRotateService {
        spire_agent_addr: format!("unix://{}", cluster.server_socket_path().display()),
        x509_context_tx,
    };

    service.stream_from_agent(CancellationToken::new()).await;

    assert!(x509_context_rx.try_recv().is_err());
}
