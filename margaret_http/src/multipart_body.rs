use std::path::Path;

use http_body_util::BodyDataStream;
use multer::Constraints;
use multer::Field;
use multer::Multipart;
use multer::SizeLimit;
use tempfile::NamedTempFile;
use tempfile::TempPath;
use tokio::io::AsyncWriteExt;

use crate::body_limit::BodyLimit;
use crate::form_field::FormField;
use crate::request_body::RequestBody;
use crate::request_error::RequestError;
use crate::request_outcome::RequestOutcome;
use crate::request_rejection::RequestRejection;
use crate::upload_config::UploadConfig;
use crate::uploaded_file::UploadedFile;

fn reject_multipart(source: multer::Error) -> RequestRejection {
    match source {
        multer::Error::StreamSizeExceeded { limit } => RequestRejection::PayloadTooLarge { limit },
        source => RequestRejection::MalformedMultipart { source },
    }
}

fn upload_write(source: std::io::Error) -> RequestError {
    RequestError::UploadWrite { source }
}

async fn write_field_to_uploaded_file(
    file: &mut tokio::fs::File,
    field: &mut Field<'_>,
    path: TempPath,
    field_name: String,
    file_name: String,
    content_type: String,
) -> Result<RequestOutcome<UploadedFile>, RequestError> {
    let mut size = 0;

    loop {
        match field.chunk().await {
            Ok(Some(chunk)) => {
                file.write_all(&chunk).await.map_err(upload_write)?;
                size += chunk.len() as u64;
            }
            Ok(None) => break,
            Err(source) => return Ok(RequestOutcome::Rejected(reject_multipart(source))),
        }
    }

    file.flush().await.map_err(upload_write)?;

    Ok(RequestOutcome::Parsed(UploadedFile::new(
        field_name,
        file_name,
        content_type,
        size,
        path,
    )))
}

async fn stream_field_to_file(
    mut field: Field<'_>,
    field_name: String,
    file_name: String,
    content_type: String,
    directory: &Path,
) -> Result<RequestOutcome<UploadedFile>, RequestError> {
    let temporary = NamedTempFile::new_in(directory)
        .map_err(|source| RequestError::UploadTempFile { source })?;
    let (file, path) = temporary.into_parts();
    let mut file = tokio::fs::File::from_std(file);
    write_field_to_uploaded_file(
        &mut file,
        &mut field,
        path,
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
    ) -> Result<RequestOutcome<Self>, RequestError> {
        let constraints =
            Constraints::new().size_limit(SizeLimit::new().whole_stream(body_limit.max_bytes()));
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
    use bytes::Bytes;
    use tempfile::NamedTempFile;

    use http_body_util::BodyDataStream;
    use http_body_util::Full;
    use multer::Multipart;

    use super::write_field_to_uploaded_file;
    use crate::request_error::RequestError;

    const UPLOADED_FILE: &[u8] =
        b"--X\r\nContent-Disposition: form-data; name=\"f\"; filename=\"a\"\r\n\r\ndata\r\n--X--\r\n";

    async fn write_uploaded_file_to_dev_full(write_buffer: usize) -> RequestError {
        let mut multipart = Multipart::new(
            BodyDataStream::new(Full::new(Bytes::from_static(UPLOADED_FILE))),
            "X",
        );
        let mut field = multipart
            .next_field()
            .await
            .expect("the field parses")
            .expect("a field is present");
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .open("/dev/full")
            .await
            .expect("/dev/full opens for writing");

        file.set_max_buf_size(write_buffer);

        let path = NamedTempFile::new()
            .expect("a temporary file")
            .into_parts()
            .1;

        write_field_to_uploaded_file(
            &mut file,
            &mut field,
            path,
            "f".to_string(),
            "a".to_string(),
            "application/octet-stream".to_string(),
        )
        .await
        .err()
        .expect("writing to /dev/full fails")
    }

    #[tokio::test]
    async fn maps_a_buffered_write_failure_to_upload_write() {
        assert!(
            write_uploaded_file_to_dev_full(UPLOADED_FILE.len())
                .await
                .to_string()
                .contains("could not be written")
        );
    }

    #[tokio::test]
    async fn maps_a_streaming_write_failure_to_upload_write() {
        assert!(
            write_uploaded_file_to_dev_full(1)
                .await
                .to_string()
                .contains("could not be written")
        );
    }
}
