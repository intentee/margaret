use multer::Field;

use crate::body_limit::BodyLimit;

pub(crate) struct UploadedPart {
    pub(crate) content_type: String,
    pub(crate) field: Field<'static>,
    pub(crate) file_name: String,
    pub(crate) limit: BodyLimit,
    pub(crate) name: String,
}
