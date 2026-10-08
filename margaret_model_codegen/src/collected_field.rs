use crate::collected_field_shape::CollectedFieldShape;
use crate::field_index::FieldIndex;

pub(crate) struct CollectedField {
    pub(crate) index: FieldIndex,
    pub(crate) name: String,
    pub(crate) nullable: bool,
    pub(crate) primary_key: bool,
    pub(crate) shape: CollectedFieldShape,
    pub(crate) unique: bool,
}
