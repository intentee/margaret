use std::sync::Arc;

use margaret_http_uploaded_file::upload_config::UploadConfig;

use crate::body_limit::BodyLimit;
use crate::router::Router;
use crate::transport_config::TransportConfig;

pub struct Server {
    address: String,
    body_limit: BodyLimit,
    name: Arc<str>,
    router: Arc<Router>,
    transport: Arc<TransportConfig>,
    upload_config: Arc<UploadConfig>,
}

impl Server {
    pub fn new(
        name: impl Into<Arc<str>>,
        address: String,
        transport: TransportConfig,
        upload_config: UploadConfig,
        body_limit: BodyLimit,
        router: Router,
    ) -> Self {
        Self {
            address,
            body_limit,
            name: name.into(),
            router: Arc::new(router),
            transport: Arc::new(transport),
            upload_config: Arc::new(upload_config),
        }
    }

    #[must_use]
    pub fn address(&self) -> &str {
        &self.address
    }

    pub(crate) fn body_limit(&self) -> BodyLimit {
        self.body_limit
    }

    pub(crate) fn name(&self) -> &Arc<str> {
        &self.name
    }

    pub(crate) fn router(&self) -> &Arc<Router> {
        &self.router
    }

    pub(crate) fn transport(&self) -> &Arc<TransportConfig> {
        &self.transport
    }

    pub(crate) fn upload_config(&self) -> &Arc<UploadConfig> {
        &self.upload_config
    }
}
