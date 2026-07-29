use tokio::sync::broadcast;
use trzcina::Service;

use crate::root_cert_store_holder::RootCertStoreHolder;
use crate::root_cert_store_service::RootCertStoreService;
use crate::svid_certified_key_holder::SvidCertifiedKeyHolder;
use crate::svid_converter_service::SvidConverterService;
use crate::svid_rotate_service::SvidRotateService;

pub struct SvidService {
    root_cert_store_holder: RootCertStoreHolder,
    spire_agent_addr: String,
    svid_certified_key_holder: SvidCertifiedKeyHolder,
}

impl SvidService {
    #[must_use]
    pub fn new(spire_agent_addr: String) -> Self {
        Self {
            root_cert_store_holder: RootCertStoreHolder::default(),
            spire_agent_addr,
            svid_certified_key_holder: SvidCertifiedKeyHolder::default(),
        }
    }

    #[must_use]
    pub fn into_common_services(self) -> Vec<Box<dyn Service>> {
        let (ca_bundle_tx, ca_bundle_rx) = broadcast::channel(1);
        let (x509_context_tx, x509_context_rx) = broadcast::channel(1);

        vec![
            Box::new(SvidRotateService {
                spire_agent_addr: self.spire_agent_addr,
                x509_context_tx,
            }),
            Box::new(SvidConverterService {
                ca_bundle_tx,
                svid_certified_key_holder: self.svid_certified_key_holder,
                x509_context_rx,
            }),
            Box::new(RootCertStoreService {
                ca_bundle_rx,
                root_cert_store_holder: self.root_cert_store_holder,
            }),
        ]
    }

    #[must_use]
    pub fn root_cert_store_holder(&self) -> RootCertStoreHolder {
        self.root_cert_store_holder.clone()
    }

    #[must_use]
    pub fn svid_certified_key_holder(&self) -> SvidCertifiedKeyHolder {
        self.svid_certified_key_holder.clone()
    }
}
