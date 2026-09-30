use margaret::framework::macros::constructor;
use margaret::framework::macros::oauth_client;
use margaret::framework::macros::singleton;
use margaret::framework::oauth_client::client_authentication::ClientAuthentication;
use margaret::framework::oauth_client::declares_oauth_client::DeclaresOAuthClient;
use margaret::framework::oauth_client::oauth_client::OAuthClient;
use margaret::framework::oauth_vocabulary::client_secret::ClientSecret;

use crate::auth::blog_client_id::BLOG_CLIENT_ID;

#[singleton]
#[oauth_client(blog, issuer = margaret)]
pub struct BlogClient {
    oauth_client: OAuthClient,
}

impl BlogClient {
    /// # Errors
    ///
    /// Returns an error when the client identifier is malformed.
    #[constructor]
    pub fn create(
        #[environment_variable(from = "MARGARET_EXAMPLE_BLOG_CLIENT_SECRET")] secret: ClientSecret,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            oauth_client: OAuthClient {
                authentication: ClientAuthentication::ClientSecretBasic(secret),
                client_id: BLOG_CLIENT_ID.parse()?,
            },
        })
    }
}

impl DeclaresOAuthClient for BlogClient {
    fn oauth_client(&self) -> &OAuthClient {
        &self.oauth_client
    }
}
