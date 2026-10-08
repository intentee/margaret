use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) struct PlannedField {
    pub(crate) concrete_path: CanonicalPath,
    pub(crate) field_name: String,
}
