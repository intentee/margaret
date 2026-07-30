use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use tokio_util::sync::CancellationToken;
use trzcina::Service;

use margaret_spiffe_svid_client::readiness_gated_service::ReadinessGatedService;
use margaret_spiffe_svid_client::svid_client_readiness::SvidClientReadiness;
use margaret_spiffe_svid_client::svid_server_cert_verifier::SvidServerCertVerifier;
use margaret_spiffe_svid_client::svid_server_cert_verifier_facade::SvidServerCertVerifierFacade;
use margaret_spiffe_svid_client_tests::recording_service::RecordingService;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;

#[tokio::test]
async fn runs_the_inner_service_once_ready() {
    install_crypto_provider();

    let facade = SvidServerCertVerifierFacade::default();
    facade.update_internal_verifier(Arc::new(
        SvidServerCertVerifier::new(build_root_cert_store_with_ca(), "example.org")
            .unwrap(),
    ));

    let ran = Arc::new(AtomicBool::new(false));
    let gated = Box::new(ReadinessGatedService::new(
        SvidClientReadiness::new(facade.subscribe()),
        RecordingService::new(ran.clone()),
    ));

    gated
        .run(CancellationToken::new())
        .await
        .expect("the gated service runs the inner service");

    assert!(ran.load(Ordering::SeqCst));
}
