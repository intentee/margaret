use margaret_http_uploaded_file::uploaded_file_error::UploadedFileError;

use crate::body_limit::BodyLimit;
use crate::body_reading::BodyReading;
use crate::collect_multipart_parts::collect_multipart_parts;
use crate::collected_parts::CollectedParts;
use crate::form_field_policy::FormFieldPolicy;
use crate::index_form_fields::index_form_fields;
use crate::index_uploaded_files::index_uploaded_files;
use crate::multipart_content::MultipartContent;
use crate::request::Request;
use crate::request_body::RequestBody;

/// # Errors
///
/// Returns `UploadedFileError` when an uploaded file cannot be written to the upload directory.
pub async fn read_multipart(
    request: &Request,
    body: RequestBody,
    limit: BodyLimit,
) -> Result<BodyReading<MultipartContent>, UploadedFileError> {
    let CollectedParts { fields, files } =
        match collect_multipart_parts(request, body, limit, FormFieldPolicy::Accepted).await? {
            BodyReading::Read(collected) => collected,
            BodyReading::Rejected(rejection) => return Ok(BodyReading::Rejected(rejection)),
        };
    let fields = match index_form_fields(fields) {
        BodyReading::Read(fields) => fields,
        BodyReading::Rejected(rejection) => return Ok(BodyReading::Rejected(rejection)),
    };

    Ok(match index_uploaded_files(files) {
        BodyReading::Read(files) => BodyReading::Read(MultipartContent { fields, files }),
        BodyReading::Rejected(rejection) => BodyReading::Rejected(rejection),
    })
}
