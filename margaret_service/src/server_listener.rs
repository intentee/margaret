use margaret_http::transport_config::TransportConfig;
use margaret_http_uploaded_file::upload_config::UploadConfig;

pub struct ServerListener {
    pub address: String,
    pub transport: TransportConfig,
    pub uploads: UploadConfig,
}
