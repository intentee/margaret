use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;

pub(crate) struct FrameworkStateStore {
    pub(crate) provided: CanonicalPath,
    pub(crate) schema: CrateRoot,
}
