use async_trait::async_trait;

use margaret_http::http_interceptor::HttpInterceptor;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::intercepts;
use margaret_macros::singleton;

use crate::views::view::View;

#[singleton]
#[intercepts]
pub struct ViewInterceptor;

#[async_trait]
impl HttpInterceptor for ViewInterceptor {
    type Intercepted = dyn View;

    async fn intercept(&self, _request: &Request, view: Box<dyn View>) -> Response {
        Response::html(200, format!("<main>{}</main>", view.body()))
    }
}
