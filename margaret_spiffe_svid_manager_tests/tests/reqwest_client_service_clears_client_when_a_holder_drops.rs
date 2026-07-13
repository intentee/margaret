use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use trzcina::Service as _;

use margaret_spiffe_svid_manager::reqwest_client_holder::ReqwestClientHolder;
use margaret_spiffe_svid_manager::reqwest_client_service::ReqwestClientService;
use margaret_spiffe_svid_manager::root_cert_store_holder::RootCertStoreHolder;
use margaret_spiffe_svid_manager::svid_certified_key_holder::SvidCertifiedKeyHolder;
use margaret_spiffe_svid_manager_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_manager_tests::build_workload_credentials::build_workload_credentials;
use margaret_spiffe_svid_manager_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_manager_tests::wait_for_reqwest_state::wait_for_reqwest_state;

#[tokio::test]
async fn builds_client_once_both_holders_populated_and_clears_when_one_drops() {
    install_crypto_provider();

    let reqwest_client_holder = ReqwestClientHolder::default();
    let root_cert_store_holder = RootCertStoreHolder::default();
    let svid_certified_key_holder = SvidCertifiedKeyHolder::default();
    let service = ReqwestClientService {
        reqwest_client_holder: reqwest_client_holder.clone(),
        root_cert_store_holder: root_cert_store_holder.clone(),
        spiffe_trust_domain: "example.org".to_string(),
        svid_certified_key_holder: svid_certified_key_holder.clone(),
    };

    let mut subscription = reqwest_client_holder.subscribe();
    subscription.read_current();

    let cancellation_token = CancellationToken::new();
    let cancellation_token_for_service = cancellation_token.clone();
    let service_task =
        tokio::spawn(async move { Box::new(service).run(cancellation_token_for_service).await });

    root_cert_store_holder.set(Some(build_root_cert_store_with_ca()));
    wait_for_reqwest_state(&mut subscription, false, &reqwest_client_holder).await;

    svid_certified_key_holder.set(Some(Arc::new(build_workload_credentials())));
    wait_for_reqwest_state(&mut subscription, true, &reqwest_client_holder).await;

    root_cert_store_holder.set(None);
    wait_for_reqwest_state(&mut subscription, false, &reqwest_client_holder).await;

    cancellation_token.cancel();
    service_task.await.unwrap().unwrap();
}
