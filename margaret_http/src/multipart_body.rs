use std::path::Path;

use http_body_util::BodyDataStream;
use multer::Constraints;
use multer::Field;
use multer::Multipart;
use multer::SizeLimit;

use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_http_uploaded_file::uploaded_file::UploadedFile;
use margaret_http_uploaded_file::uploaded_file_error::UploadedFileError;
use margaret_http_uploaded_file::uploaded_file_writer::UploadedFileWriter;

use crate::body_limit::BodyLimit;
use crate::form_field::FormField;
use crate::request_body::RequestBody;
use crate::request_outcome::RequestOutcome;
use crate::request_rejection::RequestRejection;

fn reject_multipart(source: multer::Error) -> RequestRejection {
    match source {
        multer::Error::StreamSizeExceeded { limit } => RequestRejection::PayloadTooLarge { limit },
        source => RequestRejection::MalformedMultipart { source },
    }
}

async fn stream_field_into_writer(
    mut writer: UploadedFileWriter,
    field: &mut Field<'_>,
    field_name: String,
    file_name: String,
    content_type: String,
) -> Result<RequestOutcome<UploadedFile>, UploadedFileError> {
    loop {
        match field.chunk().await {
            Ok(Some(chunk)) => writer.write_chunk(&chunk).await?,
            Ok(None) => break,
            Err(source) => return Ok(RequestOutcome::Rejected(reject_multipart(source))),
        }
    }

    Ok(RequestOutcome::Parsed(
        writer.finish(field_name, file_name, content_type).await?,
    ))
}

async fn stream_field_to_file(
    mut field: Field<'_>,
    field_name: String,
    file_name: String,
    content_type: String,
    directory: &Path,
) -> Result<RequestOutcome<UploadedFile>, UploadedFileError> {
    stream_field_into_writer(
        UploadedFileWriter::create_in(directory)?,
        &mut field,
        field_name,
        file_name,
        content_type,
    )
    .await
}

pub(crate) struct MultipartBody {
    pub(crate) files: Vec<UploadedFile>,
    pub(crate) post: Vec<FormField>,
}

impl MultipartBody {
    pub(crate) async fn parse(
        body: RequestBody,
        boundary: String,
        body_limit: &BodyLimit,
        upload_config: &UploadConfig,
    ) -> Result<RequestOutcome<Self>, UploadedFileError> {
        let constraints = Constraints::new()
            .size_limit(SizeLimit::new().whole_stream(body_limit.max_bytes() as u64));
        let mut multipart =
            Multipart::with_constraints(BodyDataStream::new(body), boundary, constraints);
        let mut files = Vec::new();
        let mut post = Vec::new();

        loop {
            let field = match multipart.next_field().await {
                Ok(Some(field)) => field,
                Ok(None) => break,
                Err(source) => return Ok(RequestOutcome::Rejected(reject_multipart(source))),
            };
            let Some(field_name) = field.name().map(str::to_string) else {
                return Ok(RequestOutcome::Rejected(
                    RequestRejection::NamelessMultipartField,
                ));
            };
            let file_name = field.file_name().map(str::to_string);
            let content_type = field
                .content_type()
                .map(|media| media.essence_str().to_string());

            match file_name {
                Some(file_name) => {
                    let directory = match upload_config {
                        UploadConfig::Enabled { directory, .. } => directory,
                        UploadConfig::Disabled => {
                            return Ok(RequestOutcome::Rejected(RequestRejection::UploadsDisabled));
                        }
                    };
                    let content_type = content_type.unwrap_or_else(|| {
                        mime::APPLICATION_OCTET_STREAM.essence_str().to_string()
                    });

                    let streamed =
                        stream_field_to_file(field, field_name, file_name, content_type, directory)
                            .await?;

                    match streamed {
                        RequestOutcome::Parsed(file) => files.push(file),
                        RequestOutcome::Rejected(rejection) => {
                            return Ok(RequestOutcome::Rejected(rejection));
                        }
                    }
                }
                None => match field.text().await {
                    Ok(value) => post.push(FormField {
                        name: field_name,
                        value,
                    }),
                    Err(source) => return Ok(RequestOutcome::Rejected(reject_multipart(source))),
                },
            }
        }

        Ok(RequestOutcome::Parsed(Self { files, post }))
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use bytes::Bytes;
    use http_body_util::BodyDataStream;
    use http_body_util::Full;
    use multer::Multipart;
    use tempfile::NamedTempFile;
    use tokio::fs::OpenOptions;

    use margaret_http_uploaded_file::uploaded_file_error::UploadedFileError;
    use margaret_http_uploaded_file::uploaded_file_writer::UploadedFileWriter;

    use super::stream_field_into_writer;

    const UPLOADED_FILE: &[u8] =
        b"--X\r\nContent-Disposition: form-data; name=\"f\"; filename=\"a\"\r\n\r\ndata\r\n--X--\r\n";

    async fn write_uploaded_file_to_dev_full(write_buffer: usize) -> UploadedFileError {
        let mut multipart = Multipart::new(
            BodyDataStream::new(Full::new(Bytes::from_static(UPLOADED_FILE))),
            "X",
        );
        let mut field = multipart
            .next_field()
            .await
            .expect("the field parses")
            .expect("a field is present");
        let mut file = OpenOptions::new()
            .write(true)
            .open("/dev/full")
            .await
            .expect("/dev/full opens for writing");

        file.set_max_buf_size(write_buffer);

        let path = NamedTempFile::new()
            .expect("a temporary file")
            .into_parts()
            .1;

        stream_field_into_writer(
            UploadedFileWriter::new(file, path),
            &mut field,
            "f".to_string(),
            "a".to_string(),
            "application/octet-stream".to_string(),
        )
        .await
        .expect_err("writing to /dev/full fails")
    }

    fn upload_write() -> UploadedFileError {
        UploadedFileError::UploadWrite {
            source: std::io::Error::other("the temporary file is full"),
        }
    }

    #[tokio::test]
    async fn maps_a_buffered_write_failure_to_upload_write() {
        assert_eq!(
            discriminant(&write_uploaded_file_to_dev_full(UPLOADED_FILE.len()).await),
            discriminant(&upload_write())
        );
    }

    #[tokio::test]
    async fn maps_a_streaming_write_failure_to_upload_write() {
        assert_eq!(
            discriminant(&write_uploaded_file_to_dev_full(1).await),
            discriminant(&upload_write())
        );
    }
}
