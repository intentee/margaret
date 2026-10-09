use std::sync::Arc;

use serde::Serialize;
use uuid::Uuid;

use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::database::database::Database;
use margaret::framework::http::request_body_chunk::RequestBodyChunk;
use margaret::framework::http::request_body_stream::RequestBodyStream;
use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::margaret::models::models_upload_upload::draft::Draft;
use crate::models::upload::Upload;

#[derive(Serialize)]
struct StoredUpload {
    id: Uuid,
}

#[singleton]
#[responds_to_http(
    max_body_bytes = 8_388_608,
    method = RouteMethod::Post,
    name = "post_upload",
    path = "/uploads",
    server = "public",
)]
pub struct PostUpload {
    database: Arc<Database>,
}

impl PostUpload {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { database })
    }

    /// # Errors
    ///
    /// Returns an error when the upload cannot be stored.
    #[process]
    pub async fn respond(&self, mut upload: RequestBodyStream) -> anyhow::Result<Response> {
        let mut content = Vec::new();

        loop {
            match upload.next_chunk().await {
                RequestBodyChunk::Data(chunk) => content.extend_from_slice(&chunk),
                RequestBodyChunk::End => {
                    return Ok(Response::json(
                        201,
                        &StoredUpload {
                            id: Upload::create(Draft { content })
                                .run(self.database.as_ref())
                                .await?
                                .id,
                        },
                    ));
                }
                RequestBodyChunk::Rejected(rejection) => return Ok(rejection.into_response()),
            }
        }
    }
}
