use margaret_http_uploaded_file::uploaded_file::UploadedFile;

use crate::named_value::NamedValue;

pub(crate) struct CollectedParts {
    pub(crate) fields: Vec<NamedValue<String>>,
    pub(crate) files: Vec<UploadedFile>,
}
