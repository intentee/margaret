use reqwest::Client;
use url::Url;

pub struct JwksClientBundleParams {
    pub http_client: Client,
    pub issuer_url: Url,
}
