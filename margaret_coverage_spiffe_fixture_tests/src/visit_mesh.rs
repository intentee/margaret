use std::sync::Arc;

use reqwest::Client;
use reqwest::StatusCode;
use tempfile::tempdir;
use tokio_util::sync::CancellationToken;
use trzcina::ServiceManager;
use trzcina::ServiceShutdownOptions;

use margaret_coverage_spiffe_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_spiffe_fixture::margaret::run::run;
use margaret_local_server_tests::reserved_address::reserved_address;
use margaret_local_server_tests::visit_served::visit_served;
use margaret_spiffe_svid::register_services::register_services;
use margaret_spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams;
use margaret_spiffe_svid_client::svid_client_bundle::SvidClientBundle;
use margaret_spiffe_svid_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_tests::spire_test_cluster_params::SpireTestClusterParams;

use crate::mesh_serve_arguments::mesh_serve_arguments;
use crate::served_mesh::ServedMesh;

async fn answers(request: reqwest::RequestBuilder) -> bool {
    request
        .send()
        .await
        .is_ok_and(|response| response.status() == StatusCode::OK)
}

/// # Panics
///
/// Panics when SPIRE cannot start or the test cannot obtain its own SVID.
pub async fn visit_mesh(visit: impl AsyncFnOnce(&ServedMesh)) -> CommandOutcome {
    let cluster = SpireTestCluster::start(
        tempdir().expect("the SPIRE directory is created"),
        SpireTestClusterParams::default(),
        reserved_address().port(),
    )
    .await
    .expect("SPIRE starts");
    let mesh = reserved_address();
    let public = reserved_address();
    let svid_params = SvidServiceBundleParams {
        spiffe_trust_domain: cluster.spiffe_trust_domain().to_string(),
        spire_agent_addr: format!("unix://{}", cluster.agent_socket_path().display()),
    };
    let serve_arguments = mesh_serve_arguments(mesh, public, &svid_params);
    let svid_bundle =
        SvidClientBundle::new(svid_params).expect("the test SVID client is configured");
    let served = ServedMesh {
        mesh,
        mesh_client: svid_bundle
            .reqwest_client()
            .expect("the test SVID client builds"),
        mesh_tls: Arc::new(svid_bundle.client_config()),
        public,
        public_client: Client::new(),
    };
    let mut svid_services = ServiceManager::default();

    register_services(&mut svid_services, svid_bundle.into_services());

    let svid_cancellation = CancellationToken::new();
    let svids = svid_services
        .start(svid_cancellation.clone())
        .run_to_completion(ServiceShutdownOptions::default());
    let relay = format!("http://{}/relay", served.public);
    let peer = format!("https://{}/peer", served.mesh);
    let outcome = visit_served(
        run(serve_arguments),
        || async {
            answers(served.public_client.get(&relay)).await
                && answers(served.mesh_client.get(&peer)).await
        },
        async || visit(&served).await,
    )
    .await;

    svid_cancellation.cancel();
    svids
        .await
        .into_result()
        .expect("the test SVID services stop cleanly");

    outcome
}
