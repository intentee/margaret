use std::sync::Arc;

use margaret_spiffe_svid_manager::ca_bundle::CaBundle;
use margaret_spiffe_svid_manager::root_cert_store_holder::RootCertStoreHolder;
use margaret_spiffe_svid_manager::root_cert_store_service::RootCertStoreService;
use margaret_spiffe_svid_manager_tests::test_fixtures::CA_DER;
use rustls::pki_types::CertificateDer;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

#[tokio::test]
async fn converts_ca_bundle_into_root_cert_store() {
    let (ca_bundle_tx, ca_bundle_rx) = broadcast::channel(1);
    let root_cert_store_holder = RootCertStoreHolder::default();
    let service = RootCertStoreService {
        ca_bundle_rx,
        root_cert_store_holder: root_cert_store_holder.clone(),
    };

    let mut subscription = root_cert_store_holder.subscribe();
    subscription.read_current();

    let cancellation_token = CancellationToken::new();
    let cancellation_token_for_service = cancellation_token.clone();
    let service_task =
        tokio::spawn(async move { Box::new(service).run(cancellation_token_for_service).await });

    let bundle = CaBundle {
        ca_certs: vec![CertificateDer::from(CA_DER.to_vec())],
    };
    ca_bundle_tx.send(Arc::new(bundle)).unwrap();

    subscription.changed().await;

    let received_store = root_cert_store_holder.get().unwrap();
    assert_eq!(received_store.roots.len(), 1);

    cancellation_token.cancel();
    service_task.await.unwrap().unwrap();
}
