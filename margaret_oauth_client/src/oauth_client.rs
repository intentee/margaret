use margaret_oauth_vocabulary::client_id::ClientId;

use crate::client_authentication::ClientAuthentication;
use crate::declares_oauth_client::DeclaresOAuthClient;

pub struct OAuthClient {
    pub authentication: ClientAuthentication,
    pub client_id: ClientId,
}

impl DeclaresOAuthClient for OAuthClient {
    fn oauth_client(&self) -> &OAuthClient {
        self
    }
}
