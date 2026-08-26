use margaret_attributes::canonical_path::CanonicalPath;
use margaret_model::on_delete::OnDelete;

pub(crate) struct DeferredModelForeignKey {
    pub(crate) columns: Vec<String>,
    pub(crate) on_delete: OnDelete,
    pub(crate) references: String,
    pub(crate) target_path: CanonicalPath,
}
