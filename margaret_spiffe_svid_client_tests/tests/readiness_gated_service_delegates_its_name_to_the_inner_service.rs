use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use trzcina::Service;

use margaret_spiffe_svid_client::readiness_gated_service::ReadinessGatedService;
use margaret_spiffe_svid_client::svid_client_readiness::SvidClientReadiness;
use margaret_spiffe_svid_client::svid_server_cert_verifier_facade::SvidServerCertVerifierFacade;
use margaret_spiffe_svid_client_tests::recording_service::RecordingService;

#[test]
fn delegates_its_name_to_the_inner_service() {
    let gated = ReadinessGatedService::new(
        SvidClientReadiness::new(SvidServerCertVerifierFacade::default().subscribe()),
        RecordingService::new(Arc::new(AtomicBool::new(false))),
    );

    assert_eq!(gated.name(), "recording_service");
}
