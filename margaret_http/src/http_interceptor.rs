use async_trait::async_trait;

use crate::http_interceptable::HttpInterceptable;
use crate::request::Request;
use crate::response::Response;

#[async_trait]
pub trait HttpInterceptor: Send + Sync {
    type Intercepted: HttpInterceptable + ?Sized;

    async fn intercept(&self, request: &Request, intercepted: Box<Self::Intercepted>) -> Response;
}
