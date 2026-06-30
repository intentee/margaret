use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::clock::Clock;

#[singleton]
#[responds_to_http(method = Get, path = "/revision", server = "public")]
pub struct GetRevision {
    clock: Arc<dyn Clock>,
}

impl GetRevision {
    #[constructor]
    pub fn create(clock: Arc<dyn Clock>) -> Self {
        Self { clock }
    }

    #[responder]
    pub async fn respond(&self) -> Response {
        Response::text(200, self.clock.revision())
    }
}
