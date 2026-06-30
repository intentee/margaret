use syn::Path;

use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) struct SiteGate {
    pub(crate) action_path: Path,
    pub(crate) gate_path: CanonicalPath,
}
