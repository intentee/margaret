use std::sync::Arc;

use async_trait::async_trait;

use margaret_http::http_middleware::HttpMiddleware;
use margaret_http::next::Next;
use margaret_http::request::Request;
use margaret_http::responded::Responded;
use margaret_macros::constructor;
use margaret_macros::handles_middleware_attribute;
use margaret_macros::singleton;

use crate::plugin::Plugin;

#[singleton]
#[handles_middleware_attribute(attribute = traced)]
pub struct Tracing {
    plugins: Vec<Arc<dyn Plugin>>,
}

impl Tracing {
    #[constructor]
    pub fn create(plugins: Vec<Arc<dyn Plugin>>) -> Self {
        Self { plugins }
    }
}

#[async_trait]
impl HttpMiddleware for Tracing {
    type Marker = ();

    async fn process(&self, request: Request, _marker: (), next: Next) -> Responded {
        let trace = self
            .plugins
            .iter()
            .map(|plugin| plugin.name())
            .collect::<Vec<String>>()
            .join(",");

        match next.run(request).await {
            Responded::Done(response) => Responded::Done(response.header("x-traced", trace)),
            outcome => outcome,
        }
    }
}
