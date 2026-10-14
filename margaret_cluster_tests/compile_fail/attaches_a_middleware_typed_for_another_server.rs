use std::convert::Infallible;
use std::sync::Arc;

use async_trait::async_trait;

use margaret::framework::handler_error::handler_error::HandlerError;
use margaret::framework::http::head_handler::HeadHandler;
use margaret::framework::http::http_middleware::HttpMiddleware;
use margaret::framework::http::layer::layer;
use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;
use margaret::framework::http::response_continuation::ResponseContinuation;
use margaret_cluster_fixture::margaret::routes::forward_targets::PublicForwardTarget;

struct IdentityGate;

#[async_trait]
impl HttpMiddleware<Infallible> for IdentityGate {
    async fn process(
        &self,
        request: &Request,
        next: Next<Infallible>,
    ) -> Result<ResponseContinuation<Infallible>, HandlerError> {
        next.run(request).await
    }
}

fn layered(
    handler: Arc<dyn HeadHandler<PublicForwardTarget>>,
) -> Arc<dyn HeadHandler<PublicForwardTarget>> {
    layer(Arc::new(IdentityGate), handler)
}

fn main() {
    drop(layered);
}
