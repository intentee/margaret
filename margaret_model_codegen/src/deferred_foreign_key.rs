use margaret_attributes::canonical_path::CanonicalPath;
use margaret_model::on_delete::OnDelete;

use crate::index_membership::IndexMembership;

pub(crate) struct DeferredForeignKey {
    pub(crate) field_name: String,
    pub(crate) indexes: Vec<IndexMembership>,
    pub(crate) nullable: bool,
    pub(crate) on_delete: OnDelete,
    pub(crate) position: usize,
    pub(crate) rust_type: String,
    pub(crate) target_path: CanonicalPath,
    pub(crate) unique: bool,
}
