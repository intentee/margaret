use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use trzcina::ServiceManager;
use trzcina::ServiceShutdownOptions;
use url::Url;

use margaret_spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams;
use margaret_spiffe_svid_bundle::svid_bundle::SvidBundle;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_tests::spire_test_cluster_params::SpireTestClusterParams;
use margaret_websocket_client::response_item::ResponseItem;
use margaret_websocket_client::web_socket_connection::WebSocketConnection;
use margaret_websocket_tests::ping_message::PingMessage;
use margaret_websocket_tests::response_chunk::ResponseChunk;
use margaret_websocket_tests::running_web_socket_server::RunningWebSocketServer;
use margaret_websocket_tests::test_session_factory::TestSessionFactory;

const SPIRE_SERVER_PORT: u16 = 20017;

#[tokio::test]
async fn answers_a_request_over_an_svid_issued_by_real_spire() {
    install_crypto_provider();

    let cluster = SpireTestCluster::start(
        tempfile::tempdir().unwrap(),
        SpireTestClusterParams::default(),
        SPIRE_SERVER_PORT,
    )
    .await
    .unwrap();
    let svid_bundle = SvidBundle::new(SvidServiceBundleParams {
        spiffe_trust_domain: cluster.spiffe_trust_domain().to_string(),
        spire_agent_addr: format!("unix://{}", cluster.agent_socket_path().display()),
    });
    let server_config = Arc::new(svid_bundle.server_config());
    let client = svid_bundle.web_socket_client();

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

    let server =
        RunningWebSocketServer::start_mutually_authenticated(TestSessionFactory, server_config)
            .await;
    let url = Url::parse(&format!("wss://localhost:{}/ws", server.address().port())).unwrap();

    let connection: WebSocketConnection = loop {
        match client.connect(&url).await {
            Ok(connection) => break connection,
            Err(_) => tokio::task::yield_now().await,
        }
    };
    let mut responses = connection
        .request::<PingMessage, ResponseChunk>(PingMessage {
            label: "spire".to_string(),
        })
        .await
        .unwrap();
    let answer = responses.next().await.unwrap().unwrap();

    assert!(matches!(answer, ResponseItem::Payload(chunk) if chunk.text == "pong spire"));

    drop(responses);
    drop(connection);
    server.stop().await;
    cancellation_token.cancel();
    manager_task.await.unwrap().into_result().unwrap();
}
