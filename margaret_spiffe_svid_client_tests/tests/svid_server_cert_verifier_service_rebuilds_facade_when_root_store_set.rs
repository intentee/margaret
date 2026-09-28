use std::sync::Arc;

use rustls::client::danger::ServerCertVerifier as _;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::ServerName;
use rustls::pki_types::UnixTime;
use tokio::task;
use tokio_util::sync::CancellationToken;
use trzcina::Service as _;

use margaret_spiffe_svid::root_cert_store_holder::RootCertStoreHolder;
use margaret_spiffe_svid_client::svid_server_cert_verifier_facade::SvidServerCertVerifierFacade;
use margaret_spiffe_svid_client::svid_server_cert_verifier_service::SvidServerCertVerifierService;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_tests::leaf_spiffe_example_org_server_der::LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER;

#[tokio::test]
async fn rebuilds_facade_when_root_store_set() {
    install_crypto_provider();

    let root_cert_store_holder = RootCertStoreHolder::default();
    let svid_server_cert_verifier_facade = Arc::new(SvidServerCertVerifierFacade::default());
    let service = SvidServerCertVerifierService {
        root_cert_store_holder: root_cert_store_holder.clone(),
        spiffe_trust_domain: "example.org".to_string(),
        svid_server_cert_verifier_facade: svid_server_cert_verifier_facade.clone(),
    };

    let cancellation_token = CancellationToken::new();
    let cancellation_token_for_service = cancellation_token.clone();
    let service_task =
        tokio::spawn(async move { Box::new(service).run(cancellation_token_for_service).await });

    root_cert_store_holder.set(Some(build_root_cert_store_with_ca()));

    let observed_facade = svid_server_cert_verifier_facade.clone();
    loop {
        if observed_facade
            .verify_server_cert(
                &CertificateDer::from(LEAF_SPIFFE_EXAMPLE_ORG_SERVER_DER.to_vec()),
                &[],
                &ServerName::try_from("ignored.example.org").unwrap(),
                &[],
                UnixTime::now(),
            )
            .is_ok()
        {
            break;
        }

        task::yield_now().await;
    }

    cancellation_token.cancel();
    service_task.await.unwrap().unwrap();
}
