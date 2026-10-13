use std::path::Path;

use margaret_http_uploaded_file::upload_config::UploadConfig;

use crate::body_reading::BodyReading;
use crate::body_rejection::BodyRejection;
use crate::request::Request;

pub(crate) fn upload_directory(request: &Request) -> BodyReading<&Path> {
    match request.server().upload_config() {
        UploadConfig::Enabled { directory } => BodyReading::Read(directory),
        UploadConfig::Disabled => BodyReading::Rejected(BodyRejection::UploadsDisabled),
    }
}
