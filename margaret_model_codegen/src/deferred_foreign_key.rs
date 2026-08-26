use margaret_attributes::canonical_path::CanonicalPath;
use margaret_model::on_delete::OnDelete;

use crate::field_index::FieldIndex;

pub(crate) struct DeferredForeignKey {
    pub(crate) field_name: String,
    pub(crate) index: FieldIndex,
    pub(crate) nullable: bool,
    pub(crate) on_delete: OnDelete,
    pub(crate) rust_type: String,
    pub(crate) target_path: CanonicalPath,
    pub(crate) unique: bool,
}
