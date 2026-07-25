use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::identity_client::IdentityClient;

#[singleton]
#[responds_to_http(method = "get", path = "/call", server = "public")]
pub struct CallRoute {
    identity_client: Arc<IdentityClient>,
}

impl CallRoute {
    #[constructor]
    #[must_use]
    pub fn create(identity_client: Arc<IdentityClient>) -> Self {
        Self { identity_client }
    }

    #[process]
    pub async fn respond(&self) -> Response {
        let _http_client = self.identity_client.http_client();

        Response::text(200, "called")
    }
}
