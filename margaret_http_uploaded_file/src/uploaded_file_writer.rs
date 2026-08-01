use std::path::Path;

use tempfile::NamedTempFile;
use tempfile::TempPath;
use tokio::fs::File;
use tokio::io::AsyncWriteExt as _;

use crate::uploaded_file::UploadedFile;
use crate::uploaded_file_error::UploadedFileError;

#[derive(Debug)]
pub struct UploadedFileWriter {
    file: File,
    path: TempPath,
    size: u64,
}

impl UploadedFileWriter {
    /// # Errors
    ///
    /// Returns `UploadedFileError::UploadTempFile`.
    pub fn create_in(directory: &Path) -> Result<Self, UploadedFileError> {
        let temporary = NamedTempFile::new_in(directory)
            .map_err(|source| UploadedFileError::UploadTempFile { source })?;
        let (file, path) = temporary.into_parts();

        Ok(Self::new(File::from_std(file), path))
    }

    #[must_use]
    pub fn new(file: File, path: TempPath) -> Self {
        Self {
            file,
            path,
            size: 0,
        }
    }

    /// # Errors
    ///
    /// Returns `UploadedFileError::UploadWrite`.
    pub async fn finish(
        mut self,
        field_name: String,
        file_name: String,
        content_type: String,
    ) -> Result<UploadedFile, UploadedFileError> {
        self.file
            .flush()
            .await
            .map_err(|source| UploadedFileError::UploadWrite { source })?;

        Ok(UploadedFile::new(
            field_name,
            file_name,
            content_type,
            self.size,
            self.path,
        ))
    }

    /// # Errors
    ///
    /// Returns `UploadedFileError::UploadWrite`.
    pub async fn write_chunk(&mut self, chunk: &[u8]) -> Result<(), UploadedFileError> {
        self.file
            .write_all(chunk)
            .await
            .map_err(|source| UploadedFileError::UploadWrite { source })?;
        self.size += chunk.len() as u64;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use tempfile::tempdir;

    use super::UploadedFileWriter;
    use crate::uploaded_file_error::UploadedFileError;

    const CONTENT: &[u8] = b"data";

    #[tokio::test]
    async fn records_the_written_size_on_the_uploaded_file() {
        let directory = tempdir().expect("a temporary directory");
        let mut writer =
            UploadedFileWriter::create_in(directory.path()).expect("the writer is created");

        writer.write_chunk(CONTENT).await.expect("the chunk writes");

        let uploaded = writer
            .finish(
                "avatar".to_string(),
                "a.png".to_string(),
                "image/png".to_string(),
            )
            .await
            .expect("the uploaded file is finished");

        assert_eq!(uploaded.size(), CONTENT.len() as u64);
        assert_eq!(uploaded.field_name(), "avatar");
        assert_eq!(uploaded.file_name(), "a.png");
        assert_eq!(uploaded.content_type(), "image/png");
    }

    #[tokio::test]
    async fn reports_a_temporary_file_that_cannot_be_created() {
        let directory = tempdir().expect("a temporary directory");
        let missing = directory.path().join("absent");
        let error = UploadedFileWriter::create_in(&missing)
            .expect_err("an absent directory cannot hold a temporary file");

        assert_eq!(
            discriminant(&error),
            discriminant(&UploadedFileError::UploadTempFile {
                source: std::io::Error::other("an unusable upload directory"),
            })
        );
    }
}
