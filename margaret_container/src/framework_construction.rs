use margaret_attributes::canonical_path::CanonicalPath;

use crate::constructor_outcome::ConstructorOutcome;
use crate::framework_dependency::FrameworkDependency;

pub enum FrameworkConstruction {
    Accessor {
        accessor: String,
        source: CanonicalPath,
    },
    Constructor {
        dependencies: Vec<FrameworkDependency>,
        is_async: bool,
        method: String,
        outcome: ConstructorOutcome,
    },
    Resolved {
        dependencies: Vec<FrameworkDependency>,
        resolver: CanonicalPath,
    },
    Unit,
}
