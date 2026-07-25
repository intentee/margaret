use margaret_attributes::canonical_path::CanonicalPath;

use crate::framework_construction::FrameworkConstruction;
use crate::framework_enablement::FrameworkEnablement;

pub struct FrameworkProvider {
    pub construction: FrameworkConstruction,
    pub enablement: FrameworkEnablement,
    pub provided: CanonicalPath,
}
