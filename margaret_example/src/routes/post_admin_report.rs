use margaret_example_macros::can;
use margaret_http::response::Response;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

#[singleton]
#[responds_to_http(method = Post, path = "/admin/report")]
#[can(crate::action::Action::Write)]
pub struct PostAdminReport;

impl PostAdminReport {
    #[responder]
    pub async fn respond(&self) -> Response {
        Response::text(200, "mutated")
    }
}
