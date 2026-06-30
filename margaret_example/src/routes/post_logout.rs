use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::security::token_actor_store::TokenActorStore;

#[singleton]
#[responds_to_http(method = Post, path = "/logout")]
pub struct PostLogout {
    store: Arc<TokenActorStore>,
}

impl PostLogout {
    #[constructor]
    pub fn create(store: Arc<TokenActorStore>) -> Self {
        Self { store }
    }

    #[responder]
    pub async fn respond(&self) -> Response {
        self.store.logout(Response::see_other("/login"))
    }
}
