use std::path::Path;

use margaret_http_uploaded_file::uploaded_file::UploadedFile;
use margaret_http_uploaded_file::uploaded_file_error::UploadedFileError;
use margaret_http_uploaded_file::uploaded_file_writer::UploadedFileWriter;

use crate::body_reading::BodyReading;
use crate::multipart_rejection::multipart_rejection;
use crate::uploaded_part::UploadedPart;

async fn stream_part_into_writer(
    mut writer: UploadedFileWriter,
    UploadedPart {
        content_type,
        mut field,
        file_name,
        limit,
        name,
    }: UploadedPart,
) -> Result<BodyReading<UploadedFile>, UploadedFileError> {
    loop {
        match field.chunk().await {
            Ok(Some(chunk)) => writer.write_chunk(&chunk).await?,
            Ok(None) => break,
            Err(source) => return Ok(BodyReading::Rejected(multipart_rejection(source, limit))),
        }
    }

    Ok(BodyReading::Read(
        writer.finish(name, file_name, content_type).await?,
    ))
}

pub(crate) async fn stream_uploaded_file(
    part: Box<UploadedPart>,
    directory: &Path,
) -> Result<BodyReading<UploadedFile>, UploadedFileError> {
    stream_part_into_writer(UploadedFileWriter::create_in(directory)?, *part).await
}

#[cfg(test)]
mod tests {
    use std::io::Error;
    use std::mem::discriminant;

    use bytes::Bytes;
    use http_body_util::BodyDataStream;
    use http_body_util::Full;
    use multer::Multipart;
    use tempfile::NamedTempFile;
    use tokio::fs::OpenOptions;

    use margaret_http_uploaded_file::uploaded_file_error::UploadedFileError;
    use margaret_http_uploaded_file::uploaded_file_writer::UploadedFileWriter;

    use super::stream_part_into_writer;
    use crate::body_limit::BodyLimit;
    use crate::uploaded_part::UploadedPart;

    const UPLOADED_FILE: &[u8] =
        b"--X\r\nContent-Disposition: form-data; name=\"f\"; filename=\"a\"\r\n\r\ndata\r\n--X--\r\n";

    async fn write_uploaded_file_to_dev_full(write_buffer: usize) -> UploadedFileError {
        let mut multipart = Multipart::new(
            BodyDataStream::new(Full::new(Bytes::from_static(UPLOADED_FILE))),
            "X",
        );
        let field = multipart
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

        stream_part_into_writer(
            UploadedFileWriter::new(file, path),
            UploadedPart {
                content_type: "application/octet-stream".to_string(),
                field,
                file_name: "a".to_string(),
                limit: BodyLimit::new(1024),
                name: "f".to_string(),
            },
        )
        .await
        .expect_err("writing to /dev/full fails")
    }

    fn upload_write() -> UploadedFileError {
        UploadedFileError::UploadWrite {
            source: Error::other("the temporary file is full"),
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
