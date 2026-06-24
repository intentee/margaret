use async_trait::async_trait;
use margaret_everything_example_macros::can;
use margaret_http::http_responder::HttpResponder;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

#[singleton]
#[responds_to_http(method = Post, path = "/admin/report")]
#[can(crate::action::Action::Write)]
pub struct PostAdminReport;

impl PostAdminReport {
    #[constructor]
    pub fn create() -> Self {
        Self
    }
}

#[async_trait]
impl HttpResponder for PostAdminReport {
    async fn respond(&self, _request: Request) -> Response {
        Response::text(200, "mutated")
    }
}

#[cfg(test)]
mod tests {
    use margaret_http::http_responder::HttpResponder;
    use margaret_http::method::Method;
    use margaret_http::request::Request;

    use super::PostAdminReport;

    #[tokio::test]
    async fn responds_with_the_mutation_result() {
        let response = PostAdminReport::create()
            .respond(Request::new(Method::Post, "/admin/report".to_string()))
            .await;

        assert_eq!(response.status(), 200);
        assert_eq!(response.body(), "mutated");
    }
}
