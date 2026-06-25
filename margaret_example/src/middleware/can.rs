use std::sync::Arc;

use async_trait::async_trait;

use margaret_http::http_middleware::HttpMiddleware;
use margaret_http::next::Next;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::http_middleware;
use margaret_macros::singleton;

use crate::action::Action;
use crate::config::Config;

#[singleton]
#[http_middleware(handles = can, priority = 100)]
pub struct Can {
    config: Arc<Config>,
}

impl Can {
    #[constructor]
    pub fn create(config: Arc<Config>) -> Self {
        Self { config }
    }
}

#[async_trait]
impl HttpMiddleware for Can {
    type Marker = Action;

    async fn process(&self, request: Request, action: Action, next: Next) -> Response {
        match action {
            Action::Read => next.run(request).await,
            Action::Write if request.header("x-authorized").is_some() => next.run(request).await,
            Action::Write => Response::text(403, format!("{}: forbidden", self.config.app_name())),
        }
    }
}
