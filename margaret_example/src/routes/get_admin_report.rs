use async_trait::async_trait;
use margaret_example_macros::can;
use margaret_example_macros::traced;
use margaret_http::http_responder::HttpResponder;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

#[singleton]
#[responds_to_http(method = Get, path = "/admin/report")]
#[can(crate::action::Action::Read)]
#[traced]
pub struct GetAdminReport;

impl GetAdminReport {
    #[constructor]
    pub fn create() -> Self {
        Self
    }
}

#[async_trait]
impl HttpResponder for GetAdminReport {
    async fn respond(&self, _request: Request) -> Response {
        Response::text(200, "report")
    }
}
