use margaret_oauth_client_codegen::oauth_client_declaration::OAuthClientDeclaration;
use margaret_oauth_vocabulary::client_id::ClientId;

use crate::bound_authorization_server::BoundAuthorizationServer;
use crate::bound_sign_in::BoundSignIn;

pub struct OAuthClientBinding<'declarations, 'index> {
    pub client: &'declarations OAuthClientDeclaration<'index>,
    pub server: BoundAuthorizationServer<'declarations, 'index>,
    pub sign_in: BoundSignIn<'declarations>,
}

impl OAuthClientBinding<'_, '_> {
    #[must_use]
    pub fn client_id(&self) -> &ClientId {
        match &self.server {
            BoundAuthorizationServer::External { client_id, .. } => client_id,
            BoundAuthorizationServer::Own { admitted, .. } => &admitted.client_id,
        }
    }
}
