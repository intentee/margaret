use crate::oauth_client::OAuthClient;

pub trait DeclaresOAuthClient: Send + Sync {
    fn oauth_client(&self) -> &OAuthClient;
}
