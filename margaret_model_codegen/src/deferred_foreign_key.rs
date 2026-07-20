use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) struct DeferredForeignKey {
    pub(crate) field_name: String,
    pub(crate) nullable: bool,
    pub(crate) rust_type: String,
    pub(crate) target_path: CanonicalPath,
}
