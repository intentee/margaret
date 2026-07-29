use reqwest::Client;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct IdentityClient {
    http_client: Client,
}

impl IdentityClient {
    #[constructor]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn create(#[spiffe_http_client] http_client: Client) -> anyhow::Result<Self> {
        Ok(Self { http_client })
    }

    #[must_use]
    pub fn http_client(&self) -> &Client {
        &self.http_client
    }
}
