use async_trait::async_trait;
use margaret_http::http_responder::HttpResponder;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http_example_macros::can;
use margaret_macros::constructor;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

#[singleton]
#[responds_to_http(method = Get, path = "/restricted")]
#[can(crate::action::Action::Write)]
pub struct GetRestricted;

impl GetRestricted {
    #[constructor]
    pub fn create() -> Self {
        Self
    }
}

#[async_trait]
impl HttpResponder for GetRestricted {
    async fn respond(&self, request: Request) -> Response {
        Response::text(200, request.path().to_string())
    }
}

#[cfg(test)]
mod tests {
    use margaret_http::http_responder::HttpResponder;
    use margaret_http::method::Method;
    use margaret_http::request::Request;

    use super::GetRestricted;

    #[tokio::test]
    async fn echoes_the_requested_path() {
        let response = GetRestricted
            .respond(Request::new(Method::Get, "/restricted".to_string()))
            .await;

        assert_eq!(response.status(), 200);
        assert_eq!(response.body(), "/restricted");
    }
}
