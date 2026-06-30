use margaret_attributes::canonical_path::CanonicalPath;

#[derive(Clone)]
pub struct SyntheticProvider {
    pub concrete_path: CanonicalPath,
    pub constructor: String,
    pub dependencies: Vec<CanonicalPath>,
    pub field_name: String,
}
