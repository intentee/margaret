use std::collections::HashMap;

use margaret_http_uploaded_file::uploaded_files::UploadedFiles;

pub struct MultipartContent {
    pub fields: HashMap<String, String>,
    pub files: UploadedFiles,
}
