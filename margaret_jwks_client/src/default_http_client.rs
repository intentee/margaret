use reqwest::Client;

#[must_use]
pub fn default_http_client() -> Client {
    Client::new()
}
