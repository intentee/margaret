use futures_util::TryFutureExt;
use reqwest::Client;
use reqwest::Response;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct MeshClient {
    http_client: Client,
}

impl MeshClient {
    #[must_use]
    #[constructor]
    pub fn create(#[spiffe_http_client] http_client: Client) -> Self {
        Self { http_client }
    }

    /// # Errors
    ///
    /// Returns an error when the mesh cannot be reached or does not answer in full.
    pub async fn fetch(&self, url: String) -> reqwest::Result<String> {
        self.http_client
            .get(url)
            .send()
            .and_then(Response::text)
            .await
    }
}
