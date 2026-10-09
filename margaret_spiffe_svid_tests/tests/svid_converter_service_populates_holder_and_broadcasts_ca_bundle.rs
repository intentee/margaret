use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

use margaret_spiffe_svid::svid_certified_key_holder::SvidCertifiedKeyHolder;
use margaret_spiffe_svid::svid_converter_service::SvidConverterService;
use margaret_spiffe_svid_tests::build_workload_x509_context::build_workload_x509_context;

#[tokio::test]
async fn populates_holder_and_broadcasts_ca_bundle() {
    let (ca_bundle_tx, mut ca_bundle_rx) = broadcast::channel(1);
    let (x509_context_tx, x509_context_rx) = broadcast::channel(1);
    let svid_certified_key_holder = SvidCertifiedKeyHolder::default();
    let service = SvidConverterService {
        ca_bundle_tx,
        svid_certified_key_holder: svid_certified_key_holder.clone(),
        x509_context_rx,
    };

    let mut subscription = svid_certified_key_holder.subscribe();
    subscription.read_current();

    let cancellation_token = CancellationToken::new();
    let cancellation_token_for_service = cancellation_token.clone();
    let service_task =
        tokio::spawn(async move { Box::new(service).run(cancellation_token_for_service).await });

    x509_context_tx
        .send(build_workload_x509_context().unwrap())
        .unwrap();

    subscription.changed().await;

    assert!(svid_certified_key_holder.get().is_some());
    let received_bundle = ca_bundle_rx.recv().await.unwrap();
    assert!(!received_bundle.ca_certs.is_empty());

    cancellation_token.cancel();
    service_task.await.unwrap().unwrap();
}
