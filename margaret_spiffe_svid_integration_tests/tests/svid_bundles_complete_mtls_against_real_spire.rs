use std::sync::Arc;

use rustls::ClientConnection;
use rustls::ServerConnection;
use rustls::pki_types::ServerName;
use tokio_util::sync::CancellationToken;
use trzcina::ServiceManager;
use trzcina::ServiceShutdownOptions;

use margaret_service::resolved_services::ResolvedServices;
use margaret_spiffe_svid::SvidServiceBundleParams;
use margaret_spiffe_svid_client::SvidClientBundle;
use margaret_spiffe_svid_server::SvidServer;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_tests::pump_tls_handshake::pump_tls_handshake;
use margaret_spiffe_svid_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_tests::spire_test_cluster_params::SpireTestClusterParams;

#[tokio::test]
async fn completes_mtls_with_a_real_spire_issued_svid() {
    install_crypto_provider();

    let cluster = SpireTestCluster::start(
        tempfile::tempdir().unwrap(),
        SpireTestClusterParams::default(),
        20016,
    )
    .await
    .unwrap();
    let spire_agent_addr = format!("unix://{}", cluster.agent_socket_path().display());

    let svid_server = SvidServer::new(
        cluster.spiffe_trust_domain().to_string(),
        spire_agent_addr.clone(),
    );
    let client_bundle = SvidClientBundle::new(SvidServiceBundleParams {
        spiffe_trust_domain: cluster.spiffe_trust_domain().to_string(),
        spire_agent_addr,
    });

    let server_config = Arc::new(svid_server.server_config());
    let client_config = Arc::new(client_bundle.client_config());

    let mut service_manager = ServiceManager::default();
    service_manager
        .register_bundle(ResolvedServices {
            services: svid_server.into_services(),
        })
        .await
        .unwrap();
    service_manager
        .register_bundle(client_bundle)
        .await
        .unwrap();

    let cancellation_token = CancellationToken::new();
    let cancellation_token_for_manager = cancellation_token.clone();
    let manager_task = tokio::spawn(async move {
        service_manager
            .start(cancellation_token_for_manager)
            .run_to_completion(ServiceShutdownOptions::default())
            .await
    });

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

        tokio::task::yield_now().await;
    }

    cancellation_token.cancel();
    manager_task.await.unwrap().into_result().unwrap();
}
