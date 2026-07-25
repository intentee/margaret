use margaret_attributes::canonical_path::CanonicalPath;

pub enum FrameworkDependency {
    Endpoint(CanonicalPath),
    Provider(CanonicalPath),
}
