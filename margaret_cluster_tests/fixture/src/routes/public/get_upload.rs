use std::sync::Arc;

use uuid::Uuid;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::stores::upload_store::UploadStore;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/uploads/{upload}", server = "public")]
pub struct GetUpload {
    uploads: Arc<UploadStore>,
}

impl GetUpload {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(uploads: Arc<UploadStore>) -> anyhow::Result<Self> {
        Ok(Self { uploads })
    }

    /// # Errors
    ///
    /// Returns an error when the upload cannot be read.
    #[process]
    pub async fn respond(
        &self,
        #[route_parameter(from = "upload")] upload: String,
    ) -> anyhow::Result<Response> {
        Ok(match Uuid::parse_str(&upload) {
            Ok(id) => match self.uploads.find(id).await? {
                Some(content) => Response::bytes(200, "application/octet-stream", content),
                None => Response::not_found(),
            },
            Err(_malformed) => Response::not_found(),
        })
    }
}
