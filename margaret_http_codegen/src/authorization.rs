use syn::Path;

use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) struct Authorization {
    pub(crate) gate: CanonicalPath,
    pub(crate) intent: Path,
}
