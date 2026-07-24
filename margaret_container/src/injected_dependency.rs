use margaret_attributes::canonical_path::CanonicalPath;

pub struct InjectedDependency {
    pub concrete: CanonicalPath,
    pub field: String,
}
