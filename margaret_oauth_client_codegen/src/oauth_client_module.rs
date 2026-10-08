use margaret_attributes::tag::Tag;
use margaret_oauth_vocabulary::client_id::ClientId;

use crate::module_sign_in::ModuleSignIn;

pub struct OAuthClientModule<'declarations> {
    pub client_id: &'declarations ClientId,
    pub sign_in: ModuleSignIn<'declarations>,
    pub tag: &'declarations Tag,
}
