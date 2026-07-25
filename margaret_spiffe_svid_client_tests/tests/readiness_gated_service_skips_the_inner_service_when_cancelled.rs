use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use tokio_util::sync::CancellationToken;
use trzcina::Service;

use margaret_spiffe_svid_client::readiness_gated_service::ReadinessGatedService;
use margaret_spiffe_svid_client::svid_client_readiness::SvidClientReadiness;
use margaret_spiffe_svid_client::svid_server_cert_verifier_facade::SvidServerCertVerifierFacade;
use margaret_spiffe_svid_client_tests::recording_service::RecordingService;

#[tokio::test]
async fn skips_the_inner_service_when_cancelled() {
    let ran = Arc::new(AtomicBool::new(false));
    let gated = Box::new(ReadinessGatedService::new(
        SvidClientReadiness::new(SvidServerCertVerifierFacade::default().subscribe()),
        RecordingService::new(ran.clone()),
    ));
    let cancellation_token = CancellationToken::new();

    cancellation_token.cancel();

    gated
        .run(cancellation_token)
        .await
        .expect("the gated service returns Ok on cancellation");

    assert!(!ran.load(Ordering::SeqCst));
}
