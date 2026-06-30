use std::sync::Arc;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::security::credential_authenticator::CredentialAuthenticator;
use crate::security::token_actor_store::TokenActorStore;
use crate::views::login::LOGIN_FORM;

#[singleton]
#[responds_to_http(method = Post, path = "/login", server = "public")]
pub struct PostLogin {
    authenticator: Arc<CredentialAuthenticator>,
    store: Arc<TokenActorStore>,
}

impl PostLogin {
    #[constructor]
    pub fn create(
        authenticator: Arc<CredentialAuthenticator>,
        store: Arc<TokenActorStore>,
    ) -> Self {
        Self {
            authenticator,
            store,
        }
    }

    #[responder]
    pub async fn respond(&self, request: &Request) -> Response {
        let (Some(username), Some(password)) = (request.form("username"), request.form("password"))
        else {
            return Response::html(401, LOGIN_FORM);
        };

        match self.authenticator.authenticate(&username, &password).await {
            Some(user) => self.store.login(Response::see_other("/account"), &user),
            None => Response::html(401, LOGIN_FORM),
        }
    }
}
