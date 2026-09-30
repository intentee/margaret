use margaret::framework::macros::constructor;
use margaret::framework::macros::oauth_client;
use margaret::framework::macros::singleton;
use margaret::framework::oauth_client::client_authentication::ClientAuthentication;
use margaret::framework::oauth_client::declares_oauth_client::DeclaresOAuthClient;
use margaret::framework::oauth_client::oauth_client::OAuthClient;
use margaret::framework::oauth_vocabulary::client_id::ClientId;
use margaret::framework::oauth_vocabulary::client_secret::ClientSecret;

#[singleton]
#[oauth_client(partner_client, issuer = partner)]
pub struct PartnerClient {
    oauth_client: OAuthClient,
}

impl PartnerClient {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        #[console_argument(from = "partner-client-id")] client_id: ClientId,
        #[console_argument(from = "partner-client-secret")] secret: ClientSecret,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            oauth_client: OAuthClient {
                authentication: ClientAuthentication::ClientSecretBasic(secret),
                client_id,
            },
        })
    }
}

impl DeclaresOAuthClient for PartnerClient {
    fn oauth_client(&self) -> &OAuthClient {
        &self.oauth_client
    }
}
