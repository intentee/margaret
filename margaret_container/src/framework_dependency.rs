use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;

pub enum FrameworkDependency {
    Endpoint(Tag),
    Provider(CanonicalPath),
}
