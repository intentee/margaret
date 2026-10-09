use margaret_oauth_vocabulary::scope::Scope;

use crate::sign_in_callback_routes::SignInCallbackRoutes;

#[derive(Debug, Eq, PartialEq)]
pub enum BoundSignIn<'declarations> {
    Available {
        callback_routes: SignInCallbackRoutes<'declarations>,
        scopes: &'declarations [Scope],
    },
    Unavailable,
}
