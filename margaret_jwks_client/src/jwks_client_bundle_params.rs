use rustls::ClientConfig;
use url::Url;

pub struct JwksClientBundleParams {
    pub client_config: ClientConfig,
    pub issuer_url: Url,
}
