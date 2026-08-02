use std::path::Path;

use tempfile::TempPath;

#[derive(Debug)]
pub struct UploadedFile {
    content_type: String,
    field_name: String,
    file_name: String,
    path: TempPath,
    size: u64,
}

impl UploadedFile {
    #[must_use]
    pub fn new(
        field_name: String,
        file_name: String,
        content_type: String,
        size: u64,
        path: TempPath,
    ) -> Self {
        Self {
            content_type,
            field_name,
            file_name,
            path,
            size,
        }
    }

    #[must_use]
    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    #[must_use]
    pub fn field_name(&self) -> &str {
        &self.field_name
    }

    #[must_use]
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    #[must_use]
    pub fn size(&self) -> u64 {
        self.size
    }
}
