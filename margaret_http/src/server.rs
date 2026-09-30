use std::sync::Arc;

use margaret_http_uploaded_file::upload_config::UploadConfig;

use crate::router::Router;
use crate::transport_config::TransportConfig;

pub struct Server {
    address: String,
    name: Arc<str>,
    router: Router,
    transport: TransportConfig,
    upload_config: UploadConfig,
}

impl Server {
    pub fn new(
        name: impl Into<Arc<str>>,
        address: String,
        transport: TransportConfig,
        upload_config: UploadConfig,
        router: Router,
    ) -> Self {
        Self {
            address,
            name: name.into(),
            router,
            transport,
            upload_config,
        }
    }

    #[must_use]
    pub fn address(&self) -> &str {
        &self.address
    }

    #[must_use]
    pub fn upload_config(&self) -> &UploadConfig {
        &self.upload_config
    }

    pub(crate) fn name(&self) -> &Arc<str> {
        &self.name
    }

    pub(crate) fn router(&self) -> &Router {
        &self.router
    }

    pub(crate) fn transport(&self) -> &TransportConfig {
        &self.transport
    }
}
