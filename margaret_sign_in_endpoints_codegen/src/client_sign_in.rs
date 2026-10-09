use margaret_tag_codegen::oauth_client_binding::OAuthClientBinding;

use crate::sign_in_service::SignInService;

pub struct ClientSignIn<'bindings, 'declarations, 'index> {
    pub binding: &'bindings OAuthClientBinding<'declarations, 'index>,
    pub service: SignInService<'declarations>,
}
