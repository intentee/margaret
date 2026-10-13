use margaret_http_uploaded_file::uploaded_file::UploadedFile;
use margaret_http_uploaded_file::uploaded_files::UploadedFiles;

use crate::body_reading::BodyReading;
use crate::body_rejection::BodyRejection;
use crate::named_value::NamedValue;
use crate::unique_index::UniqueIndex;

pub(crate) fn index_uploaded_files(files: Vec<UploadedFile>) -> BodyReading<UploadedFiles> {
    let named_files = files
        .into_iter()
        .map(|file| NamedValue {
            name: file.field_name().to_string(),
            value: file,
        })
        .collect();

    match UniqueIndex::of(named_files) {
        UniqueIndex::Indexed(indexed) => BodyReading::Read(UploadedFiles::new(indexed)),
        UniqueIndex::Duplicated { name } => {
            BodyReading::Rejected(BodyRejection::DuplicateUploadField { name })
        }
    }
}
