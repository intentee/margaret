use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) enum DependencyKind {
    Collection { trait_path: CanonicalPath },
    Single { provider_index: usize },
}
