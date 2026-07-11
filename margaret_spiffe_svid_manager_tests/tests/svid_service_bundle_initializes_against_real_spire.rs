use margaret_spiffe_svid_manager::SvidServiceBundle;
use margaret_spiffe_svid_manager::SvidServiceBundleParams;
use margaret_spiffe_svid_manager_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_manager_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_manager_tests::spire_test_cluster_params::SpireTestClusterParams;
use tokio_util::sync::CancellationToken;
use trzcina::ServiceManager;
use trzcina::ServiceShutdownOptions;

#[tokio::test]
async fn populates_reqwest_client_holder_from_real_spire() {
    install_crypto_provider();

    let cluster = SpireTestCluster::start(
        tempfile::tempdir().unwrap(),
        SpireTestClusterParams::default(),
        20016,
    )
    .await
    .unwrap();
    let bundle = SvidServiceBundle::new(SvidServiceBundleParams {
        spiffe_trust_domain: cluster.spiffe_trust_domain().to_string(),
        spire_agent_addr: format!("unix://{}", cluster.agent_socket_path().display()),
    });

    let reqwest_client_holder = bundle.reqwest_client_holder();
    let server_config_handle = bundle.server_config();

    let mut subscription = reqwest_client_holder.subscribe();
    subscription.read_current();

    let mut service_manager = ServiceManager::default();
    service_manager.register_bundle(bundle).await.unwrap();

    let cancellation_token = CancellationToken::new();
    let cancellation_token_for_manager = cancellation_token.clone();
    let manager_task = tokio::spawn(async move {
        service_manager
            .start(cancellation_token_for_manager)
            .run_to_completion(ServiceShutdownOptions::default())
            .await
    });

    loop {
        subscription.changed().await;
        if reqwest_client_holder.get().is_some() {
            break;
        }
    }

    assert!(server_config_handle.alpn_protocols.is_empty());

    cancellation_token.cancel();
    manager_task.await.unwrap().into_result().unwrap();
}
