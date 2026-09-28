use margaret_attributes::canonical_path::CanonicalPath;

pub enum FrameworkDependency {
    Provider(CanonicalPath),
    SingletonView(CanonicalPath),
    TokenIssuance,
}
