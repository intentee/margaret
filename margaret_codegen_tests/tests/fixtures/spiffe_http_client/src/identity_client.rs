use reqwest::Client;

use margaret_macros::constructor;
use margaret_macros::singleton;

#[singleton]
pub struct IdentityClient {
    http_client: Client,
}

impl IdentityClient {
    #[constructor]
    #[must_use]
    pub fn create(#[spiffe_http_client] http_client: Client) -> Self {
        Self { http_client }
    }

    #[must_use]
    pub fn http_client(&self) -> &Client {
        &self.http_client
    }
}
