use std::sync::Arc;

use crate::body_limit::BodyLimit;
use crate::router::Router;
use crate::upload_config::UploadConfig;

pub struct Server {
    address: String,
    body_limit: BodyLimit,
    name: Arc<str>,
    origin: Arc<str>,
    router: Arc<Router>,
    upload_config: Arc<UploadConfig>,
}

impl Server {
    pub fn new(
        name: impl Into<Arc<str>>,
        address: String,
        origin: impl Into<Arc<str>>,
        upload_config: UploadConfig,
        body_limit: BodyLimit,
        router: Router,
    ) -> Self {
        Self {
            address,
            body_limit,
            name: name.into(),
            origin: origin.into(),
            router: Arc::new(router),
            upload_config: Arc::new(upload_config),
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub(crate) fn body_limit(&self) -> BodyLimit {
        self.body_limit
    }

    pub(crate) fn name(&self) -> &Arc<str> {
        &self.name
    }

    pub(crate) fn origin(&self) -> &Arc<str> {
        &self.origin
    }

    pub(crate) fn router(&self) -> &Arc<Router> {
        &self.router
    }

    pub(crate) fn upload_config(&self) -> &Arc<UploadConfig> {
        &self.upload_config
    }
}
