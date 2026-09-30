use margaret::framework::http::request_body_chunk::RequestBodyChunk;
use margaret::framework::http::request_body_stream::RequestBodyStream;
use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::models::article::Article;
use crate::models::attachment_uploader::AttachmentUploader;

#[singleton]
#[responds_to_http(
    max_body_bytes = 8_388_608,
    method = "put",
    path = "/articles/{article}/attachment",
    server = "public"
)]
pub struct PutArticleAttachment;

impl PutArticleAttachment {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn respond(
        &self,
        #[authenticated_user] AttachmentUploader { subject }: AttachmentUploader,
        #[route_parameter(from = "article")] Article { title, .. }: Article,
        mut attachment: RequestBodyStream,
    ) -> anyhow::Result<Response> {
        let mut received = 0;

        loop {
            match attachment.next_chunk().await {
                RequestBodyChunk::Data(chunk) => received += chunk.len(),
                RequestBodyChunk::End => {
                    return Ok(Response::text(
                        201,
                        format!("stored {received} byte attachment for \"{title}\" from {subject}"),
                    ));
                }
                RequestBodyChunk::Rejected(rejection) => return Ok(rejection.into_response()),
            }
        }
    }
}
