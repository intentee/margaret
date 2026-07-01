use std::sync::Arc;

use async_trait::async_trait;

use crate::deferred_interception::DeferredInterception;
use crate::http_interceptor::HttpInterceptor;
use crate::request::Request;
use crate::response::Response;

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
    async fn render(self: Box<Self>, request: &Request) -> Response {
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

    use super::Interception;
    use crate::deferred_interception::DeferredInterception;
    use crate::http_interceptable::HttpInterceptable;
    use crate::http_interceptor::HttpInterceptor;
    use crate::method::Method;
    use crate::request::Request;
    use crate::response::Response;

    struct Payload {
        status: u16,
    }

    impl HttpInterceptable for Payload {}

    struct Renderer;

    #[async_trait]
    impl HttpInterceptor for Renderer {
        type Intercepted = Payload;

        async fn intercept(&self, _request: &Request, intercepted: Box<Payload>) -> Response {
            Response::text(intercepted.status, "intercepted")
        }
    }

    #[tokio::test]
    async fn renders_the_intercepted_value_through_its_interceptor() {
        let interception = Interception::new(Arc::new(Renderer), Box::new(Payload { status: 207 }));

        let response = Box::new(interception)
            .render(&Request::new(Method::Get, "/".to_string()))
            .await
            .into_http();

        assert_eq!(response.status().as_u16(), 207);
    }
}
