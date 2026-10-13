use margaret_http_uploaded_file::uploaded_file_error::UploadedFileError;
use margaret_http_uploaded_file::uploaded_files::UploadedFiles;

use crate::body_limit::BodyLimit;
use crate::body_reading::BodyReading;
use crate::collect_multipart_parts::collect_multipart_parts;
use crate::collected_parts::CollectedParts;
use crate::form_field_policy::FormFieldPolicy;
use crate::index_uploaded_files::index_uploaded_files;
use crate::request::Request;
use crate::request_body::RequestBody;

/// # Errors
///
/// Returns `UploadedFileError` when an uploaded file cannot be written to the upload directory.
pub async fn read_uploaded_files(
    request: &Request,
    body: RequestBody,
    limit: BodyLimit,
) -> Result<BodyReading<UploadedFiles>, UploadedFileError> {
    Ok(
        match collect_multipart_parts(request, body, limit, FormFieldPolicy::Refused).await? {
            BodyReading::Read(CollectedParts { files, .. }) => index_uploaded_files(files),
            BodyReading::Rejected(rejection) => BodyReading::Rejected(rejection),
        },
    )
}
