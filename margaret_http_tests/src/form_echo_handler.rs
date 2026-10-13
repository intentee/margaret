use async_trait::async_trait;
use http::header::AUTHORIZATION;
use serde_json::Value;
use serde_json::json;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::content_handler::ContentHandler;
use margaret_http::read_form_fields::read_form_fields;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

use crate::echo_wrapping::EchoWrapping;

fn wrapped(echo: Value, wrapping: &EchoWrapping) -> Value {
    match wrapping {
        EchoWrapping::AccessToken => json!({
            "access_token": echo.to_string(),
            "issued_token_type": "urn:ietf:params:oauth:token-type:access_token",
            "token_type": "Bearer",
        }),
        EchoWrapping::ActiveIntrospection => json!({
            "active": true,
            "aud": "margaret",
            "echo": echo,
        }),
        EchoWrapping::Bare => echo,
    }
}

pub struct FormEchoHandler {
    pub limit: BodyLimit,
    pub wrapping: EchoWrapping,
}

#[async_trait]
impl ContentHandler for FormEchoHandler {
    async fn handle(
        &self,
        request: &Request,
        body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        Ok(ResponseContinuation::Done(
            match read_form_fields(request, body, self.limit).await {
                BodyReading::Read(fields) => Response::json(
                    200,
                    &wrapped(
                        json!({
                            "authorization": request.inputs.server.header(&AUTHORIZATION),
                            "fields": fields,
                        }),
                        &self.wrapping,
                    ),
                ),
                BodyReading::Rejected(rejection) => rejection.into_response(),
            },
        ))
    }
}
