use std::collections::HashMap;

use crate::uploaded_file::UploadedFile;

#[derive(Debug)]
pub struct UploadedFiles {
    by_field_name: HashMap<String, UploadedFile>,
}

impl UploadedFiles {
    #[must_use]
    pub fn new(by_field_name: HashMap<String, UploadedFile>) -> Self {
        Self { by_field_name }
    }

    #[must_use]
    pub fn get(&self, field_name: &str) -> Option<&UploadedFile> {
        self.by_field_name.get(field_name)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use tempfile::NamedTempFile;

    use super::UploadedFiles;
    use crate::uploaded_file::UploadedFile;

    #[test]
    fn finds_an_uploaded_file_by_its_field_name() {
        let path = NamedTempFile::new()
            .expect("a temporary file")
            .into_temp_path();
        let files = UploadedFiles::new(HashMap::from([(
            "cover".to_string(),
            UploadedFile::new(
                "cover".to_string(),
                "cover.png".to_string(),
                "image/png".to_string(),
                3,
                path,
            ),
        )]));

        assert_eq!(
            files.get("cover").map(UploadedFile::file_name),
            Some("cover.png")
        );
        assert!(files.get("avatar").is_none());
    }
}
