use std::sync::Arc;

use async_trait::async_trait;
use serde_json::json;

use margaret_http::handler::Handler;
use margaret_http::handler_error::HandlerError;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_oidc_provider::userinfo_answer::UserinfoAnswer;
use margaret_oidc_provider::userinfo_authentication::UserinfoAuthentication;
use margaret_oidc_provider::userinfo_endpoint::UserinfoEndpoint;

pub struct UserinfoRoute {
    pub endpoint: Arc<UserinfoEndpoint>,
}

#[async_trait]
impl Handler for UserinfoRoute {
    async fn handle(
        &self,
        request: &Request,
        _body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        Ok(ResponseContinuation::Done(
            match self
                .endpoint
                .authenticate(request)
                .expect("the userinfo endpoint holds its keys")
            {
                UserinfoAuthentication::Authenticated(grant) => {
                    let UserinfoAnswer::Answered(response) = self
                        .endpoint
                        .answer(&grant, &json!({"name": "Ada"}))
                        .expect("the userinfo claims serialize")
                    else {
                        panic!("the userinfo claims are an object without a subject");
                    };

                    response
                }
                UserinfoAuthentication::Refused(response) => response,
            },
        ))
    }
}
