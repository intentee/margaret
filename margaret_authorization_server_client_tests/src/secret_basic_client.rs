use margaret_oauth_client::client_authentication::ClientAuthentication;
use margaret_oauth_client::oauth_client::OAuthClient;

/// # Panics
///
/// Panics when the fixture client identifier or secret is rejected.
#[must_use]
pub fn secret_basic_client() -> OAuthClient {
    OAuthClient {
        authentication: ClientAuthentication::ClientSecretBasic(
            "s3cret/+="
                .parse()
                .expect("the fixture client secret is visible"),
        ),
        client_id: "client:id"
            .parse()
            .expect("the fixture client identifier is visible"),
    }
}
