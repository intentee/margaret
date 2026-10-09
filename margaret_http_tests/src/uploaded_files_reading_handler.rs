use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::content_handler::ContentHandler;
use margaret_http::read_uploaded_files::read_uploaded_files;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

pub struct UploadedFilesReadingHandler {
    pub limit: BodyLimit,
}

#[async_trait]
impl ContentHandler for UploadedFilesReadingHandler {
    async fn handle(
        &self,
        request: &Request,
        body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        Ok(ResponseContinuation::Done(
            match read_uploaded_files(request, body, self.limit).await? {
                BodyReading::Read(_) => Response::text(200, "stored"),
                BodyReading::Rejected(rejection) => rejection.into_response(),
            },
        ))
    }
}
