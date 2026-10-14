use margaret_http_uploaded_file::upload_config::UploadConfig;

use crate::transport_config::TransportConfig;

pub struct Server {
    address: String,
    transport: TransportConfig,
    upload_config: UploadConfig,
}

impl Server {
    #[must_use]
    pub fn new(address: String, transport: TransportConfig, upload_config: UploadConfig) -> Self {
        Self {
            address,
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

    pub(crate) fn transport(&self) -> &TransportConfig {
        &self.transport
    }
}
