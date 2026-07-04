use std::path::Path;

use http_body_util::BodyDataStream;
use multer::Constraints;
use multer::Field;
use multer::Multipart;
use multer::SizeLimit;
use tempfile::NamedTempFile;
use tokio::io::AsyncWriteExt;

use crate::body_limit::BodyLimit;
use crate::form_field::FormField;
use crate::request_body::RequestBody;
use crate::request_error::RequestError;
use crate::upload_config::UploadConfig;
use crate::uploaded_file::UploadedFile;

fn map_multipart_error(source: multer::Error) -> RequestError {
    match source {
        multer::Error::StreamSizeExceeded { limit } => RequestError::PayloadTooLarge { limit },
        source => RequestError::Multipart { source },
    }
}

fn upload_write(source: std::io::Error) -> RequestError {
    RequestError::UploadWrite { source }
}

async fn write_field(
    file: &mut tokio::fs::File,
    field: &mut Field<'_>,
) -> Result<u64, RequestError> {
    let mut size = 0;

    while let Some(chunk) = field.chunk().await.map_err(map_multipart_error)? {
        file.write_all(&chunk).await.map_err(upload_write)?;
        size += chunk.len() as u64;
    }

    file.flush().await.map_err(upload_write)?;

    Ok(size)
}

async fn stream_field_to_file(
    mut field: Field<'_>,
    field_name: String,
    file_name: String,
    content_type: String,
    directory: &Path,
) -> Result<UploadedFile, RequestError> {
    let temporary = NamedTempFile::new_in(directory)
        .map_err(|source| RequestError::UploadTempFile { source })?;
    let (file, path) = temporary.into_parts();
    let mut file = tokio::fs::File::from_std(file);
    let size = write_field(&mut file, &mut field).await?;

    Ok(UploadedFile::new(
        field_name,
        file_name,
        content_type,
        size,
        path,
    ))
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
    ) -> Result<Self, RequestError> {
        let constraints =
            Constraints::new().size_limit(SizeLimit::new().whole_stream(body_limit.max_bytes()));
        let mut multipart =
            Multipart::with_constraints(BodyDataStream::new(body), boundary, constraints);
        let mut files = Vec::new();
        let mut post = Vec::new();

        while let Some(field) = multipart.next_field().await.map_err(map_multipart_error)? {
            let field_name = field.name().unwrap_or_default().to_string();
            let file_name = field.file_name().map(str::to_string);
            let content_type = field
                .content_type()
                .map(|media| media.essence_str().to_string());

            match file_name {
                Some(file_name) => {
                    let directory = match upload_config {
                        UploadConfig::Enabled { directory, .. } => directory,
                        UploadConfig::Disabled => return Err(RequestError::UploadsDisabled),
                    };
                    let content_type = content_type.unwrap_or_else(|| {
                        mime::APPLICATION_OCTET_STREAM.essence_str().to_string()
                    });

                    files.push(
                        stream_field_to_file(field, field_name, file_name, content_type, directory)
                            .await?,
                    );
                }
                None => {
                    let value = field.text().await.map_err(map_multipart_error)?;

                    post.push(FormField {
                        name: field_name,
                        value,
                    });
                }
            }
        }

        Ok(Self { files, post })
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use http_body_util::BodyDataStream;
    use http_body_util::Full;
    use multer::Multipart;

    use super::write_field;
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

        write_field(&mut file, &mut field)
            .await
            .expect_err("writing to /dev/full fails")
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
