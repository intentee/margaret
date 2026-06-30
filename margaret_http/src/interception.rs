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
