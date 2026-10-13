use margaret_oauth_client::client_authentication::ClientAuthentication;

use crate::fixture_client_secret::fixture_client_secret;

#[must_use]
pub fn secret_basic_authentication() -> ClientAuthentication {
    ClientAuthentication::ClientSecretBasic(fixture_client_secret())
}
