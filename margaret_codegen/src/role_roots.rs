use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) struct RoleRoots {
    pub(crate) console: Vec<CanonicalPath>,
    pub(crate) serving: Vec<CanonicalPath>,
    pub(crate) service: Vec<CanonicalPath>,
}
