use margaret_http_uploaded_file::uploaded_file_error::UploadedFileError;

use crate::body_limit::BodyLimit;
use crate::body_reading::BodyReading;
use crate::body_rejection::BodyRejection;
use crate::collected_parts::CollectedParts;
use crate::form_field_policy::FormFieldPolicy;
use crate::multipart_part::MultipartPart;
use crate::open_multipart::open_multipart;
use crate::request::Request;
use crate::request_body::RequestBody;
use crate::stream_uploaded_file::stream_uploaded_file;
use crate::upload_directory::upload_directory;

pub(crate) async fn collect_multipart_parts(
    request: &Request,
    body: RequestBody,
    limit: BodyLimit,
    form_field_policy: FormFieldPolicy,
) -> Result<BodyReading<CollectedParts>, UploadedFileError> {
    let mut parts = match open_multipart(request, body, limit) {
        BodyReading::Read(parts) => parts,
        BodyReading::Rejected(rejection) => return Ok(BodyReading::Rejected(rejection)),
    };
    let mut collected = CollectedParts {
        fields: Vec::new(),
        files: Vec::new(),
    };

    loop {
        match parts.next_part().await {
            BodyReading::Read(MultipartPart::File(part)) => {
                let directory = match upload_directory(request) {
                    BodyReading::Read(directory) => directory,
                    BodyReading::Rejected(rejection) => {
                        return Ok(BodyReading::Rejected(rejection));
                    }
                };

                match stream_uploaded_file(part, directory).await? {
                    BodyReading::Read(file) => collected.files.push(file),
                    BodyReading::Rejected(rejection) => {
                        return Ok(BodyReading::Rejected(rejection));
                    }
                }
            }
            BodyReading::Read(MultipartPart::Text(field)) => match form_field_policy {
                FormFieldPolicy::Accepted => collected.fields.push(field),
                FormFieldPolicy::Refused => {
                    return Ok(BodyReading::Rejected(BodyRejection::UnexpectedFormField {
                        name: field.name,
                    }));
                }
            },
            BodyReading::Read(MultipartPart::End) => return Ok(BodyReading::Read(collected)),
            BodyReading::Rejected(rejection) => return Ok(BodyReading::Rejected(rejection)),
        }
    }
}
