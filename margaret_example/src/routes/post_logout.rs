use std::sync::Arc;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::security::session_user_store::SessionUserStore;

#[singleton]
#[responds_to_http(method = Post, path = "/logout")]
pub struct PostLogout {
    store: Arc<SessionUserStore>,
}

impl PostLogout {
    #[constructor]
    pub fn create(store: Arc<SessionUserStore>) -> Self {
        Self { store }
    }

    #[responder]
    pub async fn respond(&self, request: &Request) -> Response {
        self.store.logout(request, Response::see_other("/login"))
    }
}
