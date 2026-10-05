use margaret_oauth_client::declares_oauth_client::DeclaresOAuthClient;
use margaret_oauth_client::oauth_client::OAuthClient;

pub struct OAuthClientDeclaration {
    pub client: OAuthClient,
}

impl DeclaresOAuthClient for OAuthClientDeclaration {
    fn oauth_client(&self) -> &OAuthClient {
        &self.client
    }
}
