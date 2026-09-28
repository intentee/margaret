use margaret_attributes::canonical_path::CanonicalPath;

#[derive(Clone)]
pub struct InjectedDependency {
    pub concrete: CanonicalPath,
    pub field: String,
}
