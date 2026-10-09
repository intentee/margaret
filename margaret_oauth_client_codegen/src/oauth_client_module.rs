use margaret_attributes::tag::Tag;
use margaret_oauth_vocabulary::client_id::ClientId;

use crate::module_sign_in::ModuleSignIn;
use crate::oauth_client_credentials::OAuthClientCredentials;

pub struct OAuthClientModule<'declarations> {
    pub client_id: &'declarations ClientId,
    pub credentials: &'declarations OAuthClientCredentials<'declarations>,
    pub sign_in: ModuleSignIn<'declarations>,
    pub tag: &'declarations Tag,
}
