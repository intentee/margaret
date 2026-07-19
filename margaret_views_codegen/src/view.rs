use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) struct View {
    pub(crate) accessor: String,
    pub(crate) concrete_path: CanonicalPath,
    pub(crate) name: String,
}
