use std::sync::Arc;

use async_trait::async_trait;

use crate::deferred_interception::DeferredInterception;
use crate::http_interceptor::HttpInterceptor;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

pub struct Interception<Interceptor>
where
    Interceptor: HttpInterceptor,
{
    intercepted: Box<Interceptor::Intercepted>,
    interceptor: Arc<Interceptor>,
}

impl<Interceptor> Interception<Interceptor>
where
    Interceptor: HttpInterceptor,
{
    pub fn new(interceptor: Arc<Interceptor>, intercepted: Box<Interceptor::Intercepted>) -> Self {
        Self {
            intercepted,
            interceptor,
        }
    }
}

#[async_trait]
impl<Interceptor> DeferredInterception for Interception<Interceptor>
where
    Interceptor: HttpInterceptor + 'static,
{
    async fn render(self: Box<Self>, request: &Request) -> ResponseContinuation {
        let Self {
            intercepted,
            interceptor,
        } = *self;

        interceptor.intercept(request, intercepted).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use http::Method;

    use super::Interception;
    use crate::handler::Handler;
    use crate::http_interceptable::HttpInterceptable;
    use crate::http_interceptor::HttpInterceptor;
    use crate::request::Request;
    use crate::respond_recursively::respond_recursively;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;
    use crate::servers::Servers;

    struct Payload {
        status: u16,
    }

    impl HttpInterceptable for Payload {}

    struct Renderer;

    #[async_trait]
    impl HttpInterceptor for Renderer {
        type Intercepted = Payload;

        async fn intercept(
            &self,
            _request: &Request,
            intercepted: Box<Payload>,
        ) -> ResponseContinuation {
            ResponseContinuation::from(Response::text(intercepted.status, "intercepted"))
        }
    }

    struct YieldsInterception;

    #[async_trait]
    impl Handler for YieldsInterception {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::Intercept(Box::new(Interception::new(
                Arc::new(Renderer),
                Box::new(Payload { status: 207 }),
            )))
        }
    }

    #[tokio::test]
    async fn renders_the_intercepted_value_through_its_interceptor() {
        let servers = Arc::new(Servers::new(Vec::new(), Vec::new()));
        let status = respond_recursively(
            &servers,
            Request::new(Method::GET, "/".to_string()),
            Arc::new(YieldsInterception),
        )
        .await
        .into_http()
        .status()
        .as_u16();

        assert_eq!(status, 207);
    }
}
