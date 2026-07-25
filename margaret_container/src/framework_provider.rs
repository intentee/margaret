use margaret_attributes::canonical_path::CanonicalPath;

use crate::framework_construction::FrameworkConstruction;
use crate::framework_enablement::FrameworkEnablement;
use crate::framework_injection_role::FrameworkInjectionRole;

pub struct FrameworkProvider {
    pub construction: FrameworkConstruction,
    pub enablement: FrameworkEnablement,
    pub injection: FrameworkInjectionRole,
    pub provided: CanonicalPath,
}
