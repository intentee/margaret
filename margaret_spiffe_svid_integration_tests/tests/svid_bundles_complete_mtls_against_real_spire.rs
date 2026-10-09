use std::sync::Arc;

use rustls::ClientConnection;
use rustls::ServerConnection;
use rustls::pki_types::ServerName;
use tokio::task;
use tokio_util::sync::CancellationToken;
use trzcina::ServiceManager;
use trzcina::ServiceShutdownOptions;

use margaret_spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams;
use margaret_spiffe_svid_bundle::svid_bundle::SvidBundle;
use margaret_spiffe_svid_tests::pump_tls_handshake::pump_tls_handshake;
use margaret_spiffe_svid_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_tests::spire_test_cluster_params::SpireTestClusterParams;
use margaret_sync_holder::sync_holder_presence::SyncHolderPresence;

#[tokio::test]
async fn completes_mtls_with_a_real_spire_issued_svid() {
    let cluster = SpireTestCluster::start(
        tempfile::tempdir().unwrap(),
        SpireTestClusterParams::default(),
        20016,
    )
    .await
    .unwrap();
    let spire_agent_addr = format!("unix://{}", cluster.agent_socket_path().display());

    let svid_bundle = SvidBundle::new(SvidServiceBundleParams {
        spiffe_trust_domain: cluster.spiffe_trust_domain().to_string(),
        spire_agent_addr,
    })
    .expect("the svid side is configured");

    let server_config = Arc::new(svid_bundle.server_config());
    let client_config = Arc::new(svid_bundle.client_config());
    let mut client_readiness = svid_bundle.client_readiness();

    let mut service_manager = ServiceManager::default();
    service_manager.register_bundle(svid_bundle).await.unwrap();

    let cancellation_token = CancellationToken::new();
    let cancellation_token_for_manager = cancellation_token.clone();
    let manager_task = tokio::spawn(async move {
        service_manager
            .start(cancellation_token_for_manager)
            .run_to_completion(ServiceShutdownOptions::default())
            .await
    });

    assert_eq!(
        client_readiness.wait_until_ready(&cancellation_token).await,
        SyncHolderPresence::Present
    );

    loop {
        let mut server_connection = ServerConnection::new(server_config.clone()).unwrap();
        let mut client_connection = ClientConnection::new(
            client_config.clone(),
            ServerName::try_from("ignored.example.org").unwrap(),
        )
        .unwrap();

        if pump_tls_handshake(&mut server_connection, &mut client_connection).is_ok() {
            break;
        }

        task::yield_now().await;
    }

    cancellation_token.cancel();
    manager_task.await.unwrap().into_result().unwrap();
}
