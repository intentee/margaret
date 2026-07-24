use margaret_attributes::canonical_path::CanonicalPath;

use crate::framework_provider_construction::FrameworkProviderConstruction;

pub struct FrameworkProvider {
    pub construction: FrameworkProviderConstruction,
    pub path: CanonicalPath,
}
